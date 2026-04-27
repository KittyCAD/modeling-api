use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Export models as a KittyCAD render packet for browser-side rendering.
pub mod export {
    use super::*;

    /// Options for exporting a render packet.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(rename = "RenderPacketExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {}
}

/// A render packet that can be consumed directly by the browser renderer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacket {
    /// Individual renderable face primitives with stable engine metadata.
    pub primitives: Vec<RenderPacketPrimitive>,
}

/// A single renderable primitive in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketPrimitive {
    /// Packed xyz positions in OpenGL/glTF coordinates and meters.
    pub positions: Vec<f32>,

    /// Packed xyz normals in OpenGL/glTF coordinates.
    pub normals: Vec<f32>,

    /// Triangle indices into the primitive-local position buffer.
    pub indices: Vec<u32>,

    /// Stable engine object UUID for the parent solid.
    pub object_id: uuid::Uuid,

    /// Stable engine body UUID for the parent solid.
    pub body_id: uuid::Uuid,

    /// Stable engine face UUID.
    pub face_id: uuid::Uuid,

    /// The face index within the solid at export time.
    pub face_index: u32,

    /// The primitive index within the generated packet.
    pub primitive_index: u32,
}
