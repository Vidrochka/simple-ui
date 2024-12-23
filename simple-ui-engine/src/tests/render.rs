use crate::{layers::{FlexDirection, FlexLayerBuilder, Layer, RectangleShapeBuilder, Shape, ShapesLayerBuilder}, render::Render, style::FillStyleBuilder, view::{PartialView, View}};


#[test]
pub fn build_render_commands_ok() {
    let mut view = View::new("Test view", 1920, 1080);

    let mut partial_view = PartialView::new();

    partial_view.add_layer(Layer::Shape(ShapesLayerBuilder::default()
        .id("2")
        .name("Shape layer 1")
        .shapes(vec![
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([2, 2])
                    .fill(FillStyleBuilder::default().color([255, 0, 0, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([14, 2])
                    .fill(FillStyleBuilder::default().color([0, 255, 0, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
        ])
        .build()
        .unwrap())
    );

    let id = partial_view.add_layer(Layer::Flex(FlexLayerBuilder::default()
        .id("3")
        .name("Flex layer 2")
        .direction(FlexDirection::Vertical)
        .gap(10u16)
        .build()
        .unwrap())
    );

    partial_view.add_child_layer(&id, Layer::Shape(ShapesLayerBuilder::default().id("2")
        .name("Shape layer 2")
        .shapes(vec![
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([2, 2])
                    .fill(FillStyleBuilder::default().color([30, 30, 0, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([14, 2])
                    .fill(FillStyleBuilder::default().color([0, 30, 30, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
        ])
        .build()
        .unwrap()
    ));

    partial_view.add_child_layer(&id, Layer::Shape(ShapesLayerBuilder::default().id("2")
        .name("Shape layer 3")
        .shapes(vec![
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([2, 2])
                    .fill(FillStyleBuilder::default().color([150, 0, 150, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
            Shape::Rectangle(
                RectangleShapeBuilder::default()
                    .size([10, 10])
                    .position([14, 2])
                    .fill(FillStyleBuilder::default().color([150, 150, 150, 255]).build().unwrap())
                    .build()
                    .unwrap()
            ),
        ])
        .build()
        .unwrap()
    ));

    let id = view.add_view_layer(Layer::Flex(FlexLayerBuilder::default()
        .id("1")
        .name("Flex layer")
        .build()
        .unwrap())
    );

    view.replace_child_layers(id, partial_view);

    let render = Render::new();

    let commands = render.build_render_command(&view);

    println!("{commands:#?}");
} 