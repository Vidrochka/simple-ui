use mint::{Vector2, Vector4};

use crate::{
    layer_id::LayerId,
    layers::{FlexDirection, Layer},
    view::View,
};

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_render_command(&self, view: &View) -> Vec<RenderCommand> {
        let mut render_command = Vec::default();

        let root_id = view.root_id();

        self.build_render_command_recursive(
            &mut render_command,
            root_id,
            view,
            Vector2::from([0, 0]),
        );

        render_command
    }

    pub fn build_render_command_recursive(
        &self,
        render_command: &mut Vec<RenderCommand>,
        layer_id: &LayerId,
        view: &View,
        offset: Vector2<u32>,
    ) -> Vector2<u32> {
        let layer = view.layer(layer_id).unwrap();

        match layer {
            Layer::Flex(flex_layer) => {
                let mut bounds = Vector2::from([0, 0]);

                for (idx, layer_id) in flex_layer.layers.iter().enumerate() {
                    match flex_layer.justify_content {
                        crate::layers::JustifyContent::Start => {
                            let layer_bounds = self.build_render_command_recursive(
                                render_command,
                                layer_id,
                                view,
                                Vector2::from([
                                    if flex_layer.direction == FlexDirection::Horizontal {
                                        bounds.x
                                    } else {
                                        0
                                    },
                                    if flex_layer.direction == FlexDirection::Vertical {
                                        bounds.y
                                    } else {
                                        0
                                    }
                                ]),
                            );

                            if flex_layer.direction == FlexDirection::Horizontal {
                                bounds.x += layer_bounds.x;

                                if layer_bounds.y > bounds.y {
                                    bounds.y = layer_bounds.y;
                                }

                                if idx != flex_layer.layers.len() - 1 {
                                    bounds.x += flex_layer.gap as u32;
                                }
                            } else {
                                bounds.y += layer_bounds.y;

                                if layer_bounds.x > bounds.x {
                                    bounds.x = layer_bounds.x;
                                }

                                if idx != flex_layer.layers.len() - 1 {
                                    bounds.y += flex_layer.gap as u32;
                                }
                            }
                        }
                        crate::layers::JustifyContent::End => todo!(),
                        crate::layers::JustifyContent::SpaceBetween => todo!(),
                    }
                }

                if let Some(fill) = &flex_layer.fill {
                    render_command.push(RenderCommand::DrawShape {
                        points: vec![
                            Vector2::from([offset.x, offset.y]),
                            Vector2::from([offset.x + bounds.x, offset.y]),
                            Vector2::from([offset.x + bounds.x, offset.y + bounds.y]),
                            Vector2::from([offset.x, offset.y + bounds.y]),
                        ],
                        color: fill.color.clone(),
                    });
                }

                bounds
            }
            Layer::Stack(stack_layer) => {
                let mut bound = Vector2::from([0, 0]);
                
                for layer_id in &stack_layer.layers {
                    //TODO: add stack size as inner bounds
                    let layer_bound = self.build_render_command_recursive(render_command, layer_id, view, offset);

                    if layer_bound.x > bound.x {
                        bound.x = layer_bound.x;
                    }

                    if layer_bound.y > bound.y {
                        bound.y = layer_bound.y;
                    }
                }

                bound
            }
            Layer::Shape(shape_layer) => {
                for shape in &shape_layer.shapes {
                    render_command.push(RenderCommand::DrawShape {
                        points: shape.get_points(&offset),
                        color: shape.get_color().clone(),
                    });
                }

                let bounds= shape_layer.bounds();

                bounds
            }
        }
    }
}

#[derive(Debug)]
pub enum RenderCommand {
    DrawShape {
        points: Vec<Vector2<u32>>,
        color: Vector4<u8>,
    },
}
