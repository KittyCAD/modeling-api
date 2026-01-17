use kittycad_modeling_cmds_macros::define_ok_modeling_cmd_response_enum;
use serde::{Deserialize, Serialize};

impl crate::ModelingCmdOutput for () {}

define_ok_modeling_cmd_response_enum! {
    /// Output from Modeling API commands.
    pub mod output {
        use kittycad_modeling_cmds_macros::ModelingCmdOutput;
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;
        use crate::shared::{
            CameraSettings,
            CameraViewState,
            BodyType,
        };

        use crate::{self as kittycad_modeling_cmds};
        use crate::{
            base64::Base64Data,
            id::ModelingCmdId,
            length_unit::LengthUnit,
            shared::{CurveType, EntityType, ExportFile, ExtrusionFaceCapType, PathCommand, Point2d, Point3d},
            units,
        };

        /// The response of the `EngineUtilEvaluatePath` endpoint
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EngineUtilEvaluatePath {
            /// The evaluated path curve position
            pub pos: Point3d<LengthUnit>,
        }

        /// The response from the `StartPath` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct StartPath {
        }

        /// The response from the `MovePathPen` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MovePathPen {
        }

        /// The response from the `ExtendPath` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ExtendPath {
        }

        /// The response from the `Extrude` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Extrude {
        }

        /// The response from the `ExtrudeToReference` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ExtrudeToReference {
        }

        /// The response from the `TwistExtrude` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct TwistExtrude {
        }

        /// The response from the `Sweep` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Sweep {
        }

        /// The response from the `Revolve` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Revolve {
        }

        /// The response from the `Solid3dShellFace` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dShellFace {
        }

        /// The response from the `Solid3dJoin` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dJoin {
        }

        /// The response from the `Solid3dGetEdgeUuid` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetEdgeUuid {
            /// The UUID of the edge.
            pub edge_id: Uuid,
        }

        /// The response from the `Solid3dGetFaceUuid` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetFaceUuid {
            /// The UUID of the face.
            pub face_id: Uuid,
        }

        /// The response from the `Solid3dGetBodyType` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetBodyType {
            /// The body type
            pub body_type: BodyType,
        }

        /// The response from the `RevolveAboutEdge` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct RevolveAboutEdge {
        }

        /// The response from the `CameraDragStart` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CameraDragStart {
        }

        /// The response from the `DefaultCameraLookAt` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraLookAt {
        }

        /// The response from the `DefaultCameraPerspectiveSettings` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraPerspectiveSettings {
        }

        /// The response from the `SelectAdd` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectAdd {
        }
        /// The response from the `SelectRemove` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectRemove {
        }

        /// The response from the `SceneClearAll` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SceneClearAll {
        }

        /// The response from the `SelectReplace` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectReplace {
        }

        /// The response from the `HighlightSetEntities` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct HighlightSetEntities {
        }

        /// The response from the `NewAnnotation` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct NewAnnotation {
        }

        /// The response from the `UpdateAnnotation` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct UpdateAnnotation {
        }

        /// The response from the `EdgeLinesVisible` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EdgeLinesVisible {
        }

        /// The response from the `ObjectVisible` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ObjectVisible {
        }

        /// The response from the `ObjectBringToFront` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ObjectBringToFront {
        }

        /// The response from the `ObjectSetMaterialParamsPbr` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ObjectSetMaterialParamsPbr {
        }

        /// The response from the `Solid2dAddHole` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid2dAddHole {
        }

        /// The response from the `Solid3dFilletEdge` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dFilletEdge {
        }

        /// The response from the `Solid3dCutEdges` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dCutEdges {
        }


        /// The response from the `SendObject` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SendObject {
        }

        /// The response from the `EntitySetOpacity` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntitySetOpacity {
        }

        /// The response from the `EntityFade` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityFade {
        }

        /// The response from the `MakePlane` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MakePlane {
        }

        /// The response from the `PlaneSetColor` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PlaneSetColor {
        }

        /// The response from the `SetTool` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetTool {
        }

        /// The response from the `MouseMove` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MouseMove {
        }

        /// The response from the `SketchModeDisable` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SketchModeDisable {
        }

        /// The response from the `EnableDryRun` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EnableDryRun {
        }

        /// The response from the `DisableDryRun` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DisableDryRun {
        }

        /// The response from the `CurveSetConstraint` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CurveSetConstraint {
        }

        /// The response from the `EnableSketchMode` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EnableSketchMode {
        }

        /// The response from the `SetBackgroundColor` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetBackgroundColor {
        }

        /// The response from the `SetCurrentToolProperties` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetCurrentToolProperties {
        }

        /// The response from the `SetDefaultSystemProperties` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetDefaultSystemProperties {
        }

        /// The response from the `MakeAxesGizmo` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MakeAxesGizmo {
        }

        /// The response from the `HandleMouseDragStart` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct HandleMouseDragStart {
        }

        /// The response from the `HandleMouseDragMove` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct HandleMouseDragMove {
        }

        /// The response from the `HandleMouseDragEnd` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct HandleMouseDragEnd {
        }

        /// The response from the `RemoveSceneObjects` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct RemoveSceneObjects {
        }

        /// The response from the `ReconfigureStream` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ReconfigureStream {
        }

        /// The response from the `SetSceneUnits` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetSceneUnits {
        }

        /// The response from the `SetSelectionType` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetSelectionType {
        }

        /// The response from the `SetSelectionFilter` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetSelectionFilter {
        }

        /// The response from the `DefaultCameraSetOrthographic` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraSetOrthographic {
        }

        /// The response from the `DefaultCameraSetPerspective` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraSetPerspective {
        }

        /// The response from the `DefaultCameraCenterToSelection` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraCenterToSelection {
        }

        /// The response from the `DefaultCameraCenterToScene` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraCenterToScene {
        }

        /// The response from the `SelectClear` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectClear {
        }

        /// The response from the `Export2d` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Export2d {
            /// The files that were exported.
            pub files: Vec<ExportFile>,
        }

        /// The response from the `Export3d` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Export3d {
            /// The files that were exported.
            pub files: Vec<ExportFile>,
        }

        /// The response from the `Export` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Export {
            /// The files that were exported.
            pub files: Vec<ExportFile>,
        }

        /// The response from the `SelectWithPoint` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectWithPoint {
            /// The UUID of the entity that was selected.
            pub entity_id: Option<Uuid>,
        }

        /// The response from the `HighlightSetEntity` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct HighlightSetEntity {
            /// The UUID of the entity that was highlighted.
            pub entity_id: Option<Uuid>,
            /// If the client sent a sequence ID with its request, the backend sends it back.
            pub sequence: Option<u32>,
        }
        /// The response from the `EntityGetChildUuid` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetChildUuid {
            /// The UUID of the child entity.
            pub entity_id: Uuid,
        }
        /// The response from the `EntityGetIndex` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetIndex {
            /// The child index of the entity.
            pub entity_index: u32,
        }
        /// The response from the `EntityGetPrimitiveIndex` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetPrimitiveIndex {
            /// The primitive index of the entity.
            pub primitive_index: u32,

            /// The type of this entity.  Helps infer whether this is an edge or a face index.
            pub entity_type: EntityType,
        }
        /// The response from the `EntityDeleteChildren` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityDeleteChildren {
        }
        /// The response from the `EntityGetNumChildren` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetNumChildren {
            /// The number of children the entity has.
            pub num: u32,
        }
        /// The response from the `EntityGetParentId` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetParentId {
            /// The UUID of the parent entity.
            pub entity_id: Uuid,
        }
        /// The response from the `EntityGetAllChildUuids` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetAllChildUuids {
            /// The UUIDs of the child entities.
            pub entity_ids: Vec<Uuid>,
        }

        /// The response from the `EntityGetSketchPaths` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetSketchPaths {
            /// The UUIDs of the sketch paths.
            pub entity_ids: Vec<Uuid>,
        }

        /// The response from the `Loft` command.
        #[derive(Debug, Serialize, Deserialize, JsonSchema, ModelingCmdOutput, Clone)]
        pub struct Loft {
            ///The UUID of the newly created solid loft.
            pub solid_id: Uuid,
        }

        /// The response from the `ClosePath` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ClosePath {
            /// The UUID of the lone face of the resulting solid2D.
            pub face_id: Uuid,
        }

        /// The response from the `CameraDragMove` command.
        /// Note this is an "unreliable" channel message, so this data may need more data like a "sequence"
        //  to work properly
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CameraDragMove {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `CameraDragEnd` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CameraDragEnd {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `DefaultCameraGetSettings` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraGetSettings {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `DefaultCameraGetView` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraGetView {
            /// Camera view state
            pub view: CameraViewState
        }

        /// The response from the `DefaultCameraSetView` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraSetView {}

        /// The response from the `DefaultCameraZoom` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraZoom {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `ZoomToFit` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ZoomToFit {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `OrientToFace` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct OrientToFace {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `ViewIsometric` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ViewIsometric {
            /// Camera settings
            pub settings: CameraSettings
        }

        /// The response from the `GetNumObjects` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct GetNumObjects {
            /// The number of objects in the scene.
            pub num_objects: u32,
        }

        /// The response from the `MakeOffsetPath` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MakeOffsetPath {
            /// If the offset path splits into multiple paths, this will contain the UUIDs of the
            /// new paths.
            /// If the offset path remains as a single path, this will be empty, and the resulting ID
            /// of the (single) new path will be the ID of the `MakeOffsetPath` command.
            pub entity_ids: Vec<Uuid>,
        }

        /// The response from the `SetObjectTransform` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetObjectTransform {}

        /// The response from the `AddHoleFromOffset` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct AddHoleFromOffset {
            /// If the offset path splits into multiple paths, this will contain the UUIDs of the
            /// new paths.
            /// If the offset path remains as a single path, this will be empty, and the resulting ID
            /// of the (single) new path will be the ID of the `AddHoleFromOffset` command.
            pub entity_ids: Vec<Uuid>,
        }

        /// The response from the `DefaultCameraFocusOn` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct DefaultCameraFocusOn { }

        /// The response from the `SelectGet` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectGet {
            /// The UUIDs of the selected entities.
            pub entity_ids: Vec<Uuid>,
        }

        /// Extrusion face info struct (useful for maintaining mappings between source path segment ids and extrusion faces)
        /// This includes the opposite and adjacent faces and edges.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetAdjacencyInfo {
            /// Details of each edge.
            pub edges: Vec<AdjacencyInfo>,
        }

        /// The response from the `Solid3dGetAllEdgeFaces` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetAllEdgeFaces {
            /// The UUIDs of the faces.
            pub faces: Vec<Uuid>,
        }

        /// The response from the `Solid3dFlip` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dFlip {
        }

        /// The response from the `Solid3dFlipFace` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dFlipFace {
        }

        /// The response from the `Solid3dGetAllOppositeEdges` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetAllOppositeEdges {
            /// The UUIDs of the edges.
            pub edges: Vec<Uuid>,
        }

        /// The response from the `Solid3dGetOppositeEdge` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetOppositeEdge {
            /// The UUID of the edge.
            pub edge: Uuid,
        }

        /// The response from the `Solid3dGetNextAdjacentEdge` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetNextAdjacentEdge {
            /// The UUID of the edge.
            pub edge: Option<Uuid>,
        }

        /// The response from the `Solid3dGetPrevAdjacentEdge` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetPrevAdjacentEdge {
            /// The UUID of the edge.
            pub edge: Option<Uuid>,
        }

        /// The response from the `Solid3DGetCommonEdge` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetCommonEdge {
            /// The UUID of the common edge, if any.
            pub edge: Option<Uuid>,
        }

        /// The response from the `GetEntityType` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct GetEntityType {
            /// The type of the entity.
            pub entity_type: EntityType,
        }

        /// The response from the `SceneGetEntityIds` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SceneGetEntityIds {
            /// The ids of the requested entities.
            pub entity_ids: Vec<Vec<Uuid>>,
        }

        /// The response from the `CurveGetControlPoints` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CurveGetControlPoints {
            /// Control points in the curve.
            pub control_points: Vec<Point3d<f64>>,
        }

        /// The response from the `ProjectEntityToPlane` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ProjectEntityToPlane {
            /// Projected points.
            pub projected_points: Vec<Point3d<f64>>,
        }

        /// The response from the `ProjectPointsToPlane` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ProjectPointsToPlane {
            /// Projected points.
            pub projected_points: Vec<Point3d<f64>>,
        }

        /// The response from the `CurveGetType` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, ModelingCmdOutput)]
        pub struct CurveGetType {
            /// Curve type
            pub curve_type: CurveType,
        }

        /// The response from the `MouseClick` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct MouseClick {
            /// Entities that are modified.
            pub entities_modified: Vec<Uuid>,
            /// Entities that are selected.
            pub entities_selected: Vec<Uuid>,
        }

        /// The response from the `TakeSnapshot` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct TakeSnapshot {
            /// Contents of the image.
            pub contents: Base64Data,
        }

        /// The response from the `PathGetInfo` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PathGetInfo {
            /// All segments in the path, in the order they were added.
            pub segments: Vec<PathSegmentInfo>,
        }

        /// Info about a path segment
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
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
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PathGetCurveUuidsForVertices {
            /// The UUIDs of the curve entities.
            pub curve_ids: Vec<Uuid>,
        }

        /// The response from the `PathGetCurveUuid` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PathGetCurveUuid {
            /// The UUID of the curve entity.
            pub curve_id: Uuid,
        }

        /// The response from the `PathGetVertexUuids` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PathGetVertexUuids {
            /// The UUIDs of the vertex entities.
            pub vertex_ids: Vec<Uuid>,
        }

        /// The response from the `PathGetSketchTargetUuid` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PathGetSketchTargetUuid {
            /// The UUID of the sketch target.
            pub target_id: Option<Uuid>,
        }

        /// Endpoints of a curve
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CurveGetEndPoints {
            /// Start
            pub start: Point3d<LengthUnit>,
            /// End
            pub end: Point3d<LengthUnit>,
        }

        /// Surface-local planar axes (if available)
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdOutput)]
        pub struct FaceIsPlanar {
            /// plane's origin
            pub origin: Option<Point3d<LengthUnit>>,

            /// plane's local x-axis
            pub x_axis: Option<Point3d<f64>>,

            /// plane's local y-axis
            pub y_axis: Option<Point3d<f64>>,

            /// plane's local z-axis (normal)
            pub z_axis: Option<Point3d<f64>>,
        }

        /// The 3D position on the surface that was evaluated
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdOutput)]
        pub struct FaceGetPosition {
            /// The 3D position on the surface that was evaluated
            pub pos: Point3d<LengthUnit>,
        }

        /// The 3D center of mass on the surface
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdOutput)]
        pub struct FaceGetCenter {
            /// The 3D position on the surface center of mass
            pub pos: Point3d<LengthUnit>,
        }

        /// The gradient (dFdu, dFdv) + normal vector on a brep face
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdOutput)]
        pub struct FaceGetGradient {
            /// dFdu
            pub df_du: Point3d<f64>,

            /// dFdv
            pub df_dv: Point3d<f64>,

            /// Normal (||dFdu x dFdv||)
            pub normal: Point3d<f64>,
        }

        /// Corresponding coordinates of given window coordinates, intersected on given plane.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct PlaneIntersectAndProject {
            /// Corresponding coordinates of given window coordinates, intersected on given plane.
            pub plane_coordinates: Option<Point2d<LengthUnit>>,
        }

        /// Data from importing the files
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ImportFiles {
            /// ID of the imported 3D models within the scene.
            pub object_id: Uuid,
        }

        /// Data from importing the files
        #[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ImportedGeometry {
            /// ID of the imported 3D models within the scene.
            pub id: Uuid,
            /// The original file paths that held the geometry.
            pub value: Vec<String>,
        }

        /// The mass response.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
        pub struct Mass {
            /// The mass.
            pub mass: f64,
            /// The output unit for the mass.
            pub output_unit: units::UnitMass,
        }

        /// The volume response.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
        pub struct Volume {
            /// The volume.
            pub volume: f64,
            /// The output unit for the volume.
            pub output_unit: units::UnitVolume,
        }

        /// The density response.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
        pub struct Density {
            /// The density.
            pub density: f64,
            /// The output unit for the density.
            pub output_unit: units::UnitDensity,
        }

        /// The surface area response.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
        pub struct SurfaceArea {
            /// The surface area.
            pub surface_area: f64,
            /// The output unit for the surface area.
            pub output_unit: units::UnitArea,
        }

        /// The center of mass response.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
        pub struct CenterOfMass {
            /// The center of mass.
            pub center_of_mass: Point3d<f64>,
            /// The output unit for the center of mass.
            pub output_unit: units::UnitLength,
        }

        /// The plane for sketch mode.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct GetSketchModePlane {
            /// The origin.
            pub origin: Point3d<LengthUnit>,
            /// The x axis.
            pub x_axis: Point3d<f64>,
            /// The y axis.
            pub y_axis: Point3d<f64>,
            /// The z axis (normal).
            pub z_axis: Point3d<f64>,
        }

        /// The response from the `EntitiesGetDistance` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityGetDistance {
            /// The minimum distance between the input entities.
            pub min_distance: LengthUnit,
            /// The maximum distance between the input entities.
            pub max_distance: LengthUnit,
        }

        /// Faces and edges id info (most used in identifying geometry in patterned and mirrored objects).
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct FaceEdgeInfo {
            /// The UUID of the object.
            pub object_id: Uuid,
            /// The faces of each object.
            pub faces: Vec<Uuid>,
            /// The edges of each object.
            pub edges: Vec<Uuid>,
        }

        /// A list of faces for a specific edge.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EdgeInfo {
            /// The UUID of the id.
            pub edge_id: Uuid,
            /// The faces of each edge.
            pub faces: Vec<Uuid>,
        }

        /// The response from the `EntityClone` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityClone {
            /// The Face and Edge Ids of the cloned entity.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityLinearPatternTransform` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityLinearPatternTransform {
            /// The Face, edge, and entity ids of the patterned entities.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub entity_face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityLinearPattern` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityLinearPattern {
            /// The Face, edge, and entity ids of the patterned entities.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub entity_face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityCircularPattern` command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityCircularPattern {
            /// The Face, edge, and entity ids of the patterned entities.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub entity_face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityMirror` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityMirror {
            /// The Face, edge, and entity ids of the patterned entities.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub entity_face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityMirrorAcrossEdge` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityMirrorAcrossEdge {
            /// The Face, edge, and entity ids of the patterned entities.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub entity_face_edge_ids: Vec<FaceEdgeInfo>,
        }

        /// The response from the `EntityMakeHelix` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityMakeHelix {
        }

        /// The response from the `EntityMakeHelixFromParams` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityMakeHelixFromParams {
        }

        /// The response from the `EntityMakeHelixFromEdge` endpoint.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct EntityMakeHelixFromEdge {
        }

        /// Extrusion face info struct (useful for maintaining mappings between source path segment ids and extrusion faces)
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct Solid3dGetExtrusionFaceInfo {
            /// Details of each face.
            pub faces: Vec<ExtrusionFaceInfo>,
        }

        /// Extrusion face info struct (useful for maintaining mappings between source path segment ids and extrusion faces)
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ExtrusionFaceInfo {
            /// Path component (curve) UUID.
            pub curve_id: Option<Uuid>,

            /// Face uuid.
            pub face_id: Option<Uuid>,

            /// Whether or not this extrusion face is a top/bottom cap face or not.
            /// Note that top/bottom cap faces will not have associated curve IDs.
            pub cap: ExtrusionFaceCapType,
        }

        /// Struct to contain the edge information of a wall of an extrude/rotate/loft/sweep.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct ComplementaryEdges {
            /// The opposite edge has no common vertices with the original edge. A wall may not
            /// have an opposite edge (i.e. a revolve that touches the axis of rotation).
            pub opposite_id: Option<Uuid>,
            /// Every edge that shared one common vertex with the original edge.
            pub adjacent_ids: Vec<Uuid>,
        }


        /// Edge info struct (useful for maintaining mappings between edges and faces and
        /// adjacent/opposite edges).
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct AdjacencyInfo {
            /// Original edge id and face info.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub original_info: Option<EdgeInfo>,
            /// Opposite edge and face info.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub opposite_info: Option<EdgeInfo>,
            /// Adjacent edge and face info.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub adjacent_info: Option<EdgeInfo>,
        }

        /// The response from the 'SetGridReferencePlane'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetGridReferencePlane {}

        /// The response from the 'BooleanUnion'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct BooleanUnion {
            /// If the operation produced just one solid, then its ID will be the
            /// ID of the modeling command request.
            /// But if any extra solids are produced, then their IDs will be included
            /// here.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub extra_solid_ids: Vec<Uuid>,
        }

        /// The response from the 'BooleanIntersection'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct BooleanIntersection {
            /// If the operation produced just one solid, then its ID will be the
            /// ID of the modeling command request.
            /// But if any extra solids are produced, then their IDs will be included
            /// here.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub extra_solid_ids: Vec<Uuid>,
        }

        /// The response from the 'BooleanSubtract'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct BooleanSubtract {
            /// If the operation produced just one solid, then its ID will be the
            /// ID of the modeling command request.
            /// But if any extra solids are produced, then their IDs will be included
            /// here.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub extra_solid_ids: Vec<Uuid>,
        }

        /// The response from the 'BooleanImprint'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct BooleanImprint {
            /// If the operation produced just one body, then its ID will be the
            /// ID of the modeling command request.
            /// But if any extra bodies are produced, then their IDs will be included
            /// here.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub extra_solid_ids: Vec<Uuid>,
        }

        /// The response from the 'SetGridScale'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetGridScale {}

        /// The response from the 'SetGridScale'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetGridAutoScale {}

        /// The response from the 'SetOrderIndependentTransparency'.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SetOrderIndependentTransparency {
            /// Is it now enabled, or disabled?
            pub enabled: bool,
        }

        /// The response from the 'CreateRegion'.
        /// The region should have an ID taken from the ID of the
        /// 'CreateRegion' modeling command.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct CreateRegion {
        }

        /// The response from the 'SelectRegionFromPoint'.
        /// If there are multiple ways to construct this region, this chooses arbitrarily.
        #[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ModelingCmdOutput)]
        pub struct SelectRegionFromPoint {
            /// The region the user clicked on.
            /// If they clicked an open space which isn't a region,
            /// this returns None.
            pub region: Option<crate::shared::SelectedRegion>,
        }

    }
}
