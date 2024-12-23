use ahash::AHashMap;
use serde::Serialize;

use crate::{
    layer_id::LayerId,
    layers::{Layer, StackLayerBuilder},
};

pub struct View {
    width: u32,
    height: u32,
    root_id: LayerId,
    layers: AHashMap<LayerId, Layer>,
}

impl View {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let stack = StackLayerBuilder::default().name(name).build().unwrap();

        Self {
            width,
            height,
            root_id: stack.id.clone(),
            layers: AHashMap::from([(stack.id.clone(), Layer::Stack(stack))]),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn root_id(&self) -> &LayerId {
        &self.root_id
    }

    pub fn layer(&self, id: &LayerId) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn add_view_layer(&mut self, layer: Layer) -> LayerId {
        let root_layer = self.layers.get_mut(&self.root_id).unwrap();

        let root_stack_layer = root_layer.as_stack_mut().unwrap();

        root_stack_layer.push_layer(layer.id().clone());

        let id = layer.id().clone();

        self.layers.insert(layer.id().clone(), layer);

        id
    }

    pub fn replace_child_layers(&mut self, layer_id: LayerId, partial_view: PartialView) {
        // TODO: add error processing
        let layer = self.layers.get_mut(&layer_id).unwrap();

        match layer {
            Layer::Flex(flex_layer) => {
                let deleted_layers =
                    flex_layer.replace_layers(partial_view.layers.keys().cloned().collect());

                deleted_layers.into_iter().for_each(|layer_id| {
                    self.layers.remove(&layer_id);
                });

                self.layers.extend(partial_view.layers.into_iter());
            }
            Layer::Stack(stack_layer) => {
                let deleted_layers =
                    stack_layer.replace_layers(partial_view.layers.keys().cloned().collect());

                deleted_layers.into_iter().for_each(|layer_id| {
                    self.layers.remove(&layer_id);
                });

                self.layers.extend(partial_view.layers.into_iter());
            }
            _ => panic!("add error processing"), //TODO: add error processing
        };
    }
}

#[derive(Debug, Serialize)]
pub struct PartialView {
    root: Vec<LayerId>,
    layers: AHashMap<LayerId, Layer>,
}

impl PartialView {
    pub fn new() -> Self {
        Self {
            root: vec![],
            layers: Default::default(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) -> LayerId {
        let id = layer.id().clone();

        self.layers.insert(layer.id().clone(), layer);

        id
    }

    pub fn add_child_layer(&mut self, parent_id: &LayerId, layer: Layer) {
        // TODO: add error processing
        let parent_layer = self.layers.get_mut(parent_id).unwrap();

        match parent_layer {
            Layer::Flex(flex_layer) => {
                flex_layer.push_layer(layer.id().clone());
            },
            Layer::Stack(stack_layer) => {
                stack_layer.push_layer(layer.id().clone());
            },
            _ => panic!("add error processing"),
        }
    }

    pub fn get_layer(&self, id: &LayerId) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: &LayerId) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }
}
