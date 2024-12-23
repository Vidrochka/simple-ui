use derive_builder::Builder;
use mint::Vector4;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct FillStyle {
    #[serde(rename = "c")]
    pub color: Vector4<u8>,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self { color: Vector4::from([0, 0, 0, 0]) }
    }
}
