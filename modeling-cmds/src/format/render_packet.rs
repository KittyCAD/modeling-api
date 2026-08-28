use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::shared::{Color, Point2d, Point3d};

/// Magic bytes at the start of every binary render packet.
pub const RENDER_PACKET_MAGIC: [u8; 8] = *b"ZOORPKT\0";

/// Current binary render packet version.
pub const RENDER_PACKET_VERSION: u32 = 1;

/// Size of the fixed binary render packet header in bytes.
pub const RENDER_PACKET_HEADER_SIZE: usize = 16;

/// Number of bytes in one interleaved surface vertex.
pub const RENDER_PACKET_VERTEX_STRIDE: u32 = 32;

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

/// Metadata for a binary render packet consumed by the browser renderer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacket {
    /// Binary render packet format version.
    pub version: u32,

    /// Layout of the interleaved surface vertex buffer.
    pub vertex_layout: RenderPacketVertexLayout,

    /// Byte ranges relative to the beginning of the packet's binary payload.
    pub sections: RenderPacketBinarySections,

    /// PBR materials keyed by the body IDs referenced by renderable primitives.
    #[serde(default)]
    pub body_materials: Vec<RenderPacketBodyMaterial>,

    /// Individual renderable face primitives with stable engine metadata.
    pub primitives: Vec<RenderPacketPrimitive>,

    /// Explicit engine-authored edge polylines with stable engine metadata.
    pub edges: Vec<RenderPacketEdge>,

    /// Explicit engine-authored sketch/wire polylines with sketch-local metadata.
    pub sketches: Vec<RenderPacketSketchSegment>,

    /// Explicit engine-authored sketch regions with stable engine metadata.
    pub regions: Vec<RenderPacketRegion>,
}

/// Layout of a surface vertex in the interleaved vertex section.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketVertexLayout {
    /// Distance in bytes between consecutive vertices.
    pub stride: u32,
    /// Byte offset of the float32x3 position.
    pub position_offset: u32,
    /// Byte offset of the float32x3 normal.
    pub normal_offset: u32,
    /// Byte offset of the float32x2 UV coordinate.
    pub uv_offset: u32,
}

/// A byte range within the binary payload following the JSON metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketBinarySection {
    /// Byte offset relative to the beginning of the binary payload.
    pub byte_offset: u32,
    /// Length of the section in bytes.
    pub byte_length: u32,
}

/// Packed numerical sections stored after the JSON metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketBinarySections {
    /// Interleaved surface vertices.
    pub vertices: RenderPacketBinarySection,
    /// Packet-wide uint32 primitive index for each surface vertex.
    pub primitive_indices: RenderPacketBinarySection,
    /// Global uint32 triangle indices.
    pub indices: RenderPacketBinarySection,
    /// Packed float32x2 trim-loop points.
    pub trim_points: RenderPacketBinarySection,
    /// Packed float32x3 edge-polyline points.
    pub edge_points: RenderPacketBinarySection,
    /// Packed float32x3 sketch-segment points.
    pub sketch_points: RenderPacketBinarySection,
    /// Packed float32x2 sketch-region points.
    pub region_points: RenderPacketBinarySection,
}

/// The PBR material assigned to a body in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketBodyMaterial {
    /// Stable engine body UUID used by renderable primitives.
    pub body_id: uuid::Uuid,

    /// Front-face PBR base color, including opacity in the alpha channel.
    pub base_color: Color,

    /// PBR metallic factor in the range 0 to 1.
    pub metalness: f32,

    /// PBR roughness factor in the range 0 to 1.
    pub roughness: f32,
}

/// A single renderable face range in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketPrimitive {
    /// First vertex in the packet-wide interleaved vertex section.
    pub first_vertex: u32,

    /// Number of vertices belonging to this face.
    pub vertex_count: u32,

    /// First index in the packet-wide index section.
    pub first_index: u32,

    /// Number of indices belonging to this face.
    pub index_count: u32,

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
    /// First float32x2 point in the packet-wide trim-point section.
    pub first_point: u32,

    /// Number of points in this closed trim loop.
    pub point_count: u32,
}

/// A single renderable edge polyline in a render packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
#[serde(rename_all = "camelCase")]
pub struct RenderPacketEdge {
    /// First float32x3 point in the packet-wide edge-point section.
    pub first_point: u32,

    /// Number of points in this edge polyline.
    pub point_count: u32,

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
    /// First float32x3 point in the packet-wide sketch-point section.
    pub first_point: u32,

    /// Number of points in this sketch segment.
    pub point_count: u32,

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
    /// First float32x2 point in the packet-wide region-point section.
    pub first_point: u32,

    /// Number of points in this loop.
    pub point_count: u32,
}
