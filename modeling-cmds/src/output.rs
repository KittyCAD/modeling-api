//! Output from Modeling API commands.
use kittycad_execution_plan_macros::ExecutionPlanValue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    base64::Base64Data,
    id::ModelingCmdId,
    shared::{CurveType, EntityType, ExportFile, ExtrusionFaceCapType, PathCommand, Point2d, Point3d},
    traits::ModelingCmdOutput,
    units,
};

/// The response from the `Export` endpoint.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Export {
    /// The files that were exported.
    pub files: Vec<ExportFile>,
}
/// The response from the `SelectWithPoint` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct SelectWithPoint {
    /// The UUID of the entity that was selected.
    pub entity_id: Option<Uuid>,
}
/// The response from the `HighlightSetEntity` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct HighlightSetEntity {
    /// The UUID of the entity that was highlighted.
    pub entity_id: Option<Uuid>,
    /// If the client sent a sequence ID with its request, the backend sends it back.
    pub sequence: Option<u32>,
}
/// The response from the `EntityGetChildUuid` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityGetChildUuid {
    /// The UUID of the child entity.
    pub entity_id: Uuid,
}
/// The response from the `EntityGetNumChildren` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityGetNumChildren {
    /// The number of children the entity has.
    pub num: u32,
}
/// The response from the `EntityGetParentId` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityGetParentId {
    /// The UUID of the parent entity.
    pub entity_id: Uuid,
}
/// The response from the `EntityGetAllChildUuids` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityGetAllChildUuids {
    /// The UUIDs of the child entities.
    pub entity_ids: Vec<Uuid>,
}

/// The response from the `SelectGet` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct SelectGet {
    /// The UUIDs of the selected entities.
    pub entity_ids: Vec<Uuid>,
}

/// The response from the `Solid3dGetAllEdgeFaces` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetAllEdgeFaces {
    /// The UUIDs of the faces.
    pub faces: Vec<Uuid>,
}

/// The response from the `Solid3dGetAllOppositeEdges` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetAllOppositeEdges {
    /// The UUIDs of the edges.
    pub edges: Vec<Uuid>,
}

/// The response from the `Solid3dGetOppositeEdge` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetOppositeEdge {
    /// The UUID of the edge.
    pub edge: Uuid,
}

/// The response from the `Solid3dGetNextAdjacentEdge` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetNextAdjacentEdge {
    /// The UUID of the edge.
    pub edge: Option<Uuid>,
}

/// The response from the `Solid3dGetPrevAdjacentEdge` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetPrevAdjacentEdge {
    /// The UUID of the edge.
    pub edge: Option<Uuid>,
}

/// The response from the `GetEntityType` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct GetEntityType {
    /// The type of the entity.
    pub entity_type: EntityType,
}
/// The response from the `CurveGetControlPoints` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct CurveGetControlPoints {
    /// Control points in the curve.
    pub control_points: Vec<Point3d>,
}

/// The response from the `CurveGetType` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Eq, PartialEq, ExecutionPlanValue)]
pub struct CurveGetType {
    /// Curve type
    pub curve_type: CurveType,
}

/// The response from the `MouseClick` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct MouseClick {
    /// Entities that are modified.
    pub entities_modified: Vec<Uuid>,
    /// Entities that are selected.
    pub entities_selected: Vec<Uuid>,
}

/// The response from the `TakeSnapshot` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct TakeSnapshot {
    /// Contents of the image.
    pub contents: Base64Data,
}

/// The response from the `PathGetInfo` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct PathGetInfo {
    /// All segments in the path, in the order they were added.
    pub segments: Vec<PathSegmentInfo>,
}

/// Info about a path segment
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct PathSegmentInfo {
    /// Which command created this path?
    /// This field is absent if the path command is not actually creating a path segment,
    /// e.g. moving the pen doesn't create a path segment.
    pub command_id: Option<ModelingCmdId>,
    /// What is the path segment?
    pub command: PathCommand,
    ///Whether or not this segment is a relative offset
    pub relative: bool,
}

/// The response from the `PathGetCurveUuidsForVertices` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct PathGetCurveUuidsForVertices {
    /// The UUIDs of the curve entities.
    pub curve_ids: Vec<Uuid>,
}

/// The response from the `PathGetVertexUuids` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct PathGetVertexUuids {
    /// The UUIDs of the vertex entities.
    pub vertex_ids: Vec<Uuid>,
}

/// Endpoints of a curve
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct CurveGetEndPoints {
    /// Start
    pub start: Point3d<f64>,
    /// End
    pub end: Point3d<f64>,
}

/// Corresponding coordinates of given window coordinates, intersected on given plane.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct PlaneIntersectAndProject {
    /// Corresponding coordinates of given window coordinates, intersected on given plane.
    pub plane_coordinates: Option<Point2d<f64>>,
}

/// Data from importing the files
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct ImportFiles {
    /// ID of the imported 3D models within the scene.
    pub object_id: Uuid,
}

/// The mass response.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Mass {
    /// The mass.
    pub mass: f64,
    /// The output unit for the mass.
    pub output_unit: units::UnitMass,
}

/// The volume response.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Volume {
    /// The volume.
    pub volume: f64,
    /// The output unit for the volume.
    pub output_unit: units::UnitVolume,
}

/// The density response.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Density {
    /// The density.
    pub density: f64,
    /// The output unit for the density.
    pub output_unit: units::UnitDensity,
}

/// The surface area response.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct SurfaceArea {
    /// The surface area.
    pub surface_area: f64,
    /// The output unit for the surface area.
    pub output_unit: units::UnitArea,
}

/// The center of mass response.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct CenterOfMass {
    /// The center of mass.
    pub center_of_mass: Point3d<f64>,
    /// The output unit for the center of mass.
    pub output_unit: units::UnitLength,
}

/// The plane for sketch mode.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct GetSketchModePlane {
    /// The x axis.
    pub x_axis: Point3d<f64>,
    /// The y axis.
    pub y_axis: Point3d<f64>,
    /// The z axis (normal).
    pub z_axis: Point3d<f64>,
}

/// The response from the `EntitiesGetDistance` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityGetDistance {
    /// The minimum distance between the input entities.
    pub min_distance: f64,
    /// The maximum distance between the input entities.
    pub max_distance: f64,
}

/// The response from the `EntityLinearPattern` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityLinearPattern {
    /// The UUIDs of the entities that were created.
    pub entity_ids: Vec<Uuid>,
}

/// The response from the `EntityCircularPattern` command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct EntityCircularPattern {
    /// The UUIDs of the entities that were created.
    pub entity_ids: Vec<Uuid>,
}

/// Extrusion face info struct (useful for maintaining mappings between source path segment ids and extrusion faces)
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Solid3dGetExtrusionFaceInfo {
    /// Details of each face.
    pub faces: Vec<ExtrusionFaceInfo>,
}

/// Extrusion face info struct (useful for maintaining mappings between source path segment ids and extrusion faces)
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct ExtrusionFaceInfo {
    /// Path component (curve) UUID.
    pub curve_id: Option<Uuid>,

    /// Face uuid.
    pub face_id: Option<Uuid>,

    /// Whether or not this extrusion face is a top/bottom cap face or not.
    /// Note that top/bottom cap faces will not have associated curve IDs.
    pub cap: ExtrusionFaceCapType,
}

impl ModelingCmdOutput for Export {}
impl ModelingCmdOutput for SelectWithPoint {}
impl ModelingCmdOutput for HighlightSetEntity {}
impl ModelingCmdOutput for EntityGetChildUuid {}
impl ModelingCmdOutput for EntityGetNumChildren {}
impl ModelingCmdOutput for EntityGetParentId {}
impl ModelingCmdOutput for EntityGetAllChildUuids {}
impl ModelingCmdOutput for EntityGetDistance {}
impl ModelingCmdOutput for EntityLinearPattern {}
impl ModelingCmdOutput for EntityCircularPattern {}
impl ModelingCmdOutput for SelectGet {}
impl ModelingCmdOutput for GetEntityType {}
impl ModelingCmdOutput for Solid3dGetAllEdgeFaces {}
impl ModelingCmdOutput for Solid3dGetAllOppositeEdges {}
impl ModelingCmdOutput for Solid3dGetOppositeEdge {}
impl ModelingCmdOutput for Solid3dGetPrevAdjacentEdge {}
impl ModelingCmdOutput for Solid3dGetNextAdjacentEdge {}
impl ModelingCmdOutput for MouseClick {}
impl ModelingCmdOutput for CurveGetType {}
impl ModelingCmdOutput for CurveGetControlPoints {}
impl ModelingCmdOutput for TakeSnapshot {}
impl ModelingCmdOutput for PathGetInfo {}
impl ModelingCmdOutput for PathGetCurveUuidsForVertices {}
impl ModelingCmdOutput for PathGetVertexUuids {}
impl ModelingCmdOutput for PlaneIntersectAndProject {}
impl ModelingCmdOutput for CurveGetEndPoints {}
impl ModelingCmdOutput for ImportFiles {}
impl ModelingCmdOutput for Mass {}
impl ModelingCmdOutput for Volume {}
impl ModelingCmdOutput for Density {}
impl ModelingCmdOutput for SurfaceArea {}
impl ModelingCmdOutput for CenterOfMass {}
impl ModelingCmdOutput for GetSketchModePlane {}
impl ModelingCmdOutput for () {}
impl ModelingCmdOutput for Solid3dGetExtrusionFaceInfo {}
