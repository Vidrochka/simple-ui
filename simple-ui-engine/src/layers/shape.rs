use derive_builder::Builder;
use mint::{Vector2, Vector4};
use serde::Serialize;

use crate::{layer_id::LayerId, style::FillStyle};


#[derive(Debug, Serialize, Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct ShapesLayer {
    #[serde(rename = "id")]
    #[builder(default)]
    pub id: LayerId,
    #[serde(rename = "nm")]
    #[builder(default)]
    pub name: String,
    #[serde(rename = "sh")]
    #[builder(default)]
    pub shapes: Vec<Shape>
}

impl ShapesLayer {
    pub fn bounds(&self) -> Vector2<u32> {
        let mut max = Vector2::from([0, 0]);

        for shape in &self.shapes {
            let bounds = shape.get_bounds();

            if bounds.x > max.x {
                max.x = bounds.x;
            }

            if bounds.y > max.y {
                max.y = bounds.y;
            }
        }

        max
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "ty")]
pub enum Shape {
    #[serde(rename = "rc")]
    Rectangle(RectangleShape)
}

impl Shape {
    pub fn get_points(&self, offset: &Vector2<u32>) -> Vec<Vector2<u32>> {
        match self {
            Shape::Rectangle(rect) => {
                let points = vec![
                    Vector2::from([rect.position.x + offset.x, rect.position.y + offset.y]),
                    Vector2::from([rect.position.x + rect.size.x + offset.x, rect.position.y + offset.y]),
                    Vector2::from([rect.position.x + rect.size.x + offset.x, rect.position.y + rect.size.y + offset.y]),
                    Vector2::from([rect.position.x + offset.x, rect.position.y + rect.size.y + offset.y]),
                ];

                points
            }
        }
    }

    pub fn get_color(&self) -> &Vector4<u8> {
        match self {
            Shape::Rectangle(rect) => &rect.fill.color
        }
    }

    pub fn get_bounds(&self) -> Vector2<u32> {
        match self {
            Shape::Rectangle(rect) => {
                Vector2::from([rect.size.x + rect.position.x, rect.size.y + rect.position.y])
            }
        }
    }
}

#[derive(Debug, Serialize, Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct RectangleShape {
    #[serde(rename = "p")]
    pub position: Vector2<u32>,
    #[serde(rename = "s")]
    pub size: Vector2<u32>,
    #[serde(rename = "fill")]
    pub fill: FillStyle,
}