use mint::{Vector2, Vector4};

use crate::{
    layer_id::LayerId,
    layers::{FlexDirection, FlexLayer, Layer, ShapesLayer, StackLayer},
    view::View,
};

pub struct CommandRender {}

impl CommandRender {
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
                self.build_flex_commands(flex_layer, render_command, view, offset)
            }
            Layer::Stack(stack_layer) => {
                self.build_stack_commands(stack_layer, render_command, view, offset)
            }
            Layer::Shape(shape_layer) => {
                self.build_shape_commands(shape_layer, render_command, offset)
            }
        }
    }

    fn build_flex_commands(
        &self,
        flex_layer: &FlexLayer,
        render_command: &mut Vec<RenderCommand>,
        view: &View,
        offset: Vector2<u32>,
    ) -> Vector2<u32> {
        let mut bounds = Vector2::from([0, 0]);

        for (idx, layer_id) in flex_layer.layers.iter().enumerate() {
            match flex_layer.justify_content {
                crate::layers::JustifyContent::Start => {
                    let layer_offset = match flex_layer.direction {
                        FlexDirection::Horizontal => Vector2::from([bounds.x, 0]),
                        FlexDirection::Vertical => Vector2::from([0, bounds.y]),
                    };

                    let layer_bounds = self.build_render_command_recursive(
                        render_command,
                        layer_id,
                        view,
                        layer_offset,
                    );

                    match flex_layer.direction {
                        FlexDirection::Horizontal => {
                            bounds.x += layer_bounds.x;
                            bounds.y = bounds.y.max(layer_bounds.y);
                            
                            if idx != flex_layer.layers.len() - 1 {
                                bounds.x += flex_layer.gap as u32;
                            }
                        }
                        FlexDirection::Vertical => {
                            bounds.y += layer_bounds.y;
                            bounds.x = bounds.x.max(layer_bounds.x);

                            if idx != flex_layer.layers.len() - 1 {
                                bounds.y += flex_layer.gap as u32;
                            }
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

    fn build_shape_commands(
        &self,
        shape_layer: &ShapesLayer,
        render_command: &mut Vec<RenderCommand>,
        offset: Vector2<u32>,
    ) -> Vector2<u32> {
        for shape in &shape_layer.shapes {
            render_command.push(RenderCommand::DrawShape {
                points: shape.get_points(&offset),
                color: shape.get_color().clone(),
            });
        }

        shape_layer.bounds()
    }

    fn build_stack_commands(
        &self,
        stack_layer: &StackLayer,
        render_command: &mut Vec<RenderCommand>,
        view: &View,
        offset: Vector2<u32>,
    ) -> Vector2<u32> {
        let mut bounds = Vector2::from([0, 0]);

        for layer_id in &stack_layer.layers {
            //TODO: add stack size as inner bounds
            let layer_bounds =
                self.build_render_command_recursive(render_command, layer_id, view, offset);

            bounds.x = bounds.x.max(layer_bounds.x);
            bounds.y = bounds.y.max(layer_bounds.y);
        }

        bounds
    }
}

#[derive(Debug)]
pub enum RenderCommand {
    DrawShape {
        points: Vec<Vector2<u32>>,
        color: Vector4<u8>,
    },
}
