use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::shared::{Point2d, Point3d};

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

    /// Explicit engine-authored edge polylines with stable engine metadata.
    pub edges: Vec<RenderPacketEdge>,

    /// Explicit engine-authored sketch/wire polylines with sketch-local metadata.
    pub sketches: Vec<RenderPacketSketchSegment>,

    /// Explicit engine-authored sketch regions with stable engine metadata.
    pub regions: Vec<RenderPacketRegion>,
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

    /// Packed uv coordinates in normalized face-local trim space.
    pub uvs: Vec<f32>,

    /// Triangle indices into the primitive-local position buffer.
    pub indices: Vec<u32>,

    /// Trim loops in the same normalized face-local uv space as `uvs`.
    pub trim_loops: Vec<RenderPacketTrimLoop>,

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

/// A single trim loop in normalized face-local uv space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketTrimLoop {
    /// Packed uv positions for a closed trim loop in normalized face-local space.
    pub positions: Vec<f32>,
}

/// A single renderable edge polyline in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketEdge {
    /// Packed xyz positions in OpenGL/glTF coordinates and meters.
    pub positions: Vec<f32>,

    /// Stable engine object UUID for the parent solid.
    pub object_id: uuid::Uuid,

    /// Stable engine body UUID for the parent solid.
    pub body_id: uuid::Uuid,

    /// Stable engine edge UUID.
    pub edge_id: uuid::Uuid,

    /// The edge index within the solid at export time.
    pub edge_index: u32,
}

/// A single renderable sketch/wire polyline in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketSketchSegment {
    /// Packed xyz positions in OpenGL/glTF coordinates and meters.
    pub positions: Vec<f32>,

    /// Stable engine scene object UUID for the sketch owner.
    pub sketch_id: uuid::Uuid,

    /// Stable artifact/entity UUID for the underlying sketch segment, when available.
    pub segment_id: Option<uuid::Uuid>,

    /// Curve index within the sketch path or hole loop.
    pub segment_index: u32,

    /// Hole index when this segment belongs to a hole loop.
    pub hole_index: Option<u32>,

    /// Whether the underlying curve is closed.
    pub closed: bool,
}

/// A single implicit sketch region in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketRegion {
    /// The sketch plane origin in OpenGL/glTF world coordinates and meters.
    pub plane_origin: Point3d<f32>,

    /// The sketch plane x axis in OpenGL/glTF world coordinates.
    pub plane_x_axis: Point3d<f32>,

    /// The sketch plane y axis in OpenGL/glTF world coordinates.
    pub plane_y_axis: Point3d<f32>,

    /// The explicit outer loop for this region in sketch-plane local meters.
    pub outer_loop: RenderPacketRegionLoop,

    /// Hole loops for this region in sketch-plane local meters.
    pub hole_loops: Vec<RenderPacketRegionLoop>,

    /// Stable engine scene object UUID for the sketch owner.
    pub sketch_id: uuid::Uuid,

    /// Stable engine region UUID.
    pub region_id: uuid::Uuid,

    /// Stable engine parent path UUID. This mirrors `entity_get_parent_id`.
    pub parent_id: uuid::Uuid,

    /// A point guaranteed to be inside the region, in engine millimeters.
    pub query_point: Point2d<f64>,
}

/// A single 2D loop in sketch-plane local meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketRegionLoop {
    /// Packed xy positions in sketch-plane local meters.
    pub positions: Vec<f32>,
}
