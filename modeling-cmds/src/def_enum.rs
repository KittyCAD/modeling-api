use kittycad_execution_plan_macros::{ExecutionPlanFromMemory, ExecutionPlanValue};
use kittycad_modeling_cmds_macros::define_modeling_cmd_enum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use self::each_cmd::*;
use crate::{self as kittycad_modeling_cmds};

define_modeling_cmd_enum! {
    pub mod each_cmd {
        use std::collections::HashSet;

        use crate::{self as kittycad_modeling_cmds};
        use kittycad_execution_plan_macros::{ExecutionPlanFromMemory, ExecutionPlanValue};
        use kittycad_modeling_cmds_macros::{ModelingCmdVariant, ModelingCmdVariantEmpty};
        use parse_display_derive::{Display, FromStr};
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;

        use crate::{
            format::OutputFormat,
            id::ModelingCmdId,
            length_unit::LengthUnit,
            shared::{
                Angle,
                AnnotationOptions, AnnotationType, CameraDragInteractionType, Color, DistanceType, EntityType,
                PathComponentConstraintBound, PathComponentConstraintType, PathSegment, PerspectiveCameraParameters,
                Point2d, Point3d, SceneSelectionType, SceneToolType,
            },
            units,
        };

        /// Mike says this usually looks nice.
        fn default_animation_seconds() -> f32 {
            0.4
        }

        /// Start a new path.
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct StartPath;

        /// Move the path's "pen".
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct MovePathPen {
            /// The ID of the command which created the path.
            pub path: ModelingCmdId,
            /// Where the path's pen should be.
            pub to: Point3d<LengthUnit>,
        }

        /// Extend a path by adding a new segment which starts at the path's "pen".
        /// If no "pen" location has been set before (via `MovePen`), then the pen is at the origin.
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct ExtendPath {
            /// The ID of the command which created the path.
            pub path: ModelingCmdId,
            /// Segment to append to the path.
            /// This segment will implicitly begin at the current "pen" location.
            pub segment: PathSegment,
        }

        /// Command for extruding a solid 2d.
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct Extrude {
            /// Which sketch to extrude.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// How far off the plane to extrude
            pub distance: LengthUnit,
            /// Whether to cap the extrusion with a face, or not.
            /// If true, the resulting solid will be closed on all sides, like a dice.
            /// If false, it will be open on one side, like a drinking glass.
            pub cap: bool,
        }

        /// Command for revolving a solid 2d.
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct Revolve {
            /// Which sketch to revolve.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// The origin of the extrusion axis
            pub origin: Point3d<f64>,
            /// The axis of the extrusion (taken from the origin)
            pub axis: Point3d<f64>,
            /// The signed angle of revolution (in degrees, must be <= 360 in either direction)
            pub angle: Angle,
        }

        /// Command for revolving a solid 2d about a brep edge
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct RevolveAboutEdge {
            /// Which sketch to revolve.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// The edge to use as the axis of revolution, must be linear and lie in the plane of the solid
            pub edge_id: Uuid,
            /// The signed angle of revolution (in degrees, must be <= 360 in either direction)
            pub angle: Angle,
        }

        /// Closes a path, converting it to a 2D solid.
        #[derive(
            Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct ClosePath {
            /// Which path to close.
            pub path_id: Uuid,
        }

        /// Camera drag started.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct CameraDragStart {
            /// The type of camera drag interaction.
            pub interaction: CameraDragInteractionType,
            /// The initial mouse position.
            pub window: Point2d,
        }

        /// Camera drag continued.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CameraDragMove {
            /// The type of camera drag interaction.
            pub interaction: CameraDragInteractionType,
            /// The current mouse position.
            pub window: Point2d,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Camera drag ended
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CameraDragEnd {
            /// The type of camera drag interaction.
            pub interaction: CameraDragInteractionType,
            /// The final mouse position.
            pub window: Point2d,
        }

        /// Gets the default camera's camera settings
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct DefaultCameraGetSettings;

        /// Change what the default camera is looking at.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct DefaultCameraLookAt {
            /// Where the camera is positioned
            pub vantage: Point3d,
            /// What the camera is looking at. Center of the camera's field of vision
            pub center: Point3d,
            /// Which way is "up", from the camera's point of view.
            pub up: Point3d,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Change what the default camera is looking at.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct DefaultCameraPerspectiveSettings {
            /// Where the camera is positioned
            pub vantage: Point3d,
            /// What the camera is looking at. Center of the camera's field of vision
            pub center: Point3d,
            /// Which way is "up", from the camera's point of view.
            pub up: Point3d,
            /// The field of view angle in the y direction, in degrees.
            pub fov_y: f32,
            /// The distance to the near clipping plane.
            pub z_near: f32,
            /// The distance to the far clipping plane.
            pub z_far: f32,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Adjust zoom of the default camera.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct DefaultCameraZoom {
            /// Move the camera forward along the vector it's looking at,
            /// by this magnitudedefaultCameraZoom.
            /// Basically, how much should the camera move forward by.
            pub magnitude: f32,
        }

        /// Enable sketch mode, where users can sketch 2D geometry.
        /// Users choose a plane to sketch on.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct DefaultCameraEnableSketchMode {
            /// What's the origin of the sketching plane?
            pub origin: Point3d,
            /// Which 3D axis of the scene should be the X axis of the sketching plane?
            pub x_axis: Point3d,
            /// Which 3D axis of the scene should be the Y axis of the sketching plane?
            pub y_axis: Point3d,
            /// How far to the sketching plane?
            pub distance_to_plane: f32,
            /// Should the camera use orthographic projection?
            /// In other words, should an object's size in the rendered image stay constant regardless of its distance from the camera.
            pub ortho: bool,
            /// Should we animate or snap for the camera transition?
            pub animated: bool,
        }

        /// Disable sketch mode, from the default camera.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct DefaultCameraDisableSketchMode;

        /// Export the scene to a file.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        pub struct Export {
            /// IDs of the entities to be exported. If this is empty, then all entities are exported.
            pub entity_ids: Vec<Uuid>,
            /// The file format to export to.
            pub format: OutputFormat,
        }

        /// What is this entity's parent?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityGetParentId {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// How many children does the entity have?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityGetNumChildren {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// What is the UUID of this entity's n-th child?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityGetChildUuid {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
            /// Index into the entity's list of children.
            pub child_index: u32,
        }

        /// What are all UUIDs of this entity's children?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityGetAllChildUuids {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// What is the distance between these two entities?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityGetDistance {
            /// ID of the first entity being queried.
            pub entity_id1: Uuid,
            /// ID of the second entity being queried.
            pub entity_id2: Uuid,
            /// Type of distance to be measured.
            pub distance_type: DistanceType,
        }

        /// Create a linear pattern using this entity.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityLinearPattern {
            /// ID of the entity being copied.
            pub entity_id: Uuid,
            /// Axis along which to make the copies.
            /// For Solid2d patterns, the z component is ignored.
            pub axis: Point3d<f64>,
            /// Number of repetitions to make.
            pub num_repetitions: u32,
            /// Spacing between repetitions.
            pub spacing: LengthUnit,
        }
        /// Create a circular pattern using this entity.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct EntityCircularPattern {
            /// ID of the entity being copied.
            pub entity_id: Uuid,
            /// Axis around which to make the copies.
            /// For Solid2d patterns, this is ignored.
            pub axis: Point3d<f64>,
            /// Point around which to make the copies.
            /// For Solid2d patterns, the z component is ignored.
            pub center: Point3d<LengthUnit>,
            /// Number of repetitions to make.
            pub num_repetitions: u32,
            /// Arc angle (in degrees) to place repetitions along.
            pub arc_degrees: f64,
            /// Whether or not to rotate the objects as they are copied.
            pub rotate_duplicates: bool,
        }

        /// Create a helix using the input cylinder and other specified parameters.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdVariantEmpty)]
        pub struct EntityMakeHelix {
            /// ID of the cylinder.
            pub cylinder_id: Uuid,
            /// Number of revolutions.
            pub revolutions: f64,
            /// Start angle (in degrees).
            pub start_angle: Angle,
            /// Is the helix rotation clockwise?
            pub is_clockwise: bool,
            /// Length of the helix.
            pub length: LengthUnit,
        }

        /// Enter edit mode
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct EditModeEnter {
            /// The edit target
            pub target: Uuid,
        }

        /// Modifies the selection by simulating a "mouse click" at the given x,y window coordinate
        /// Returns ID of whatever was selected.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct SelectWithPoint {
            /// Where in the window was selected
            pub selected_at_window: Point2d,
            /// What entity was selected?
            pub selection_type: SceneSelectionType,
        }

        /// Adds one or more entities (by UUID) to the selection.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SelectAdd {
            /// Which entities to select
            pub entities: Vec<Uuid>,
        }

        /// Removes one or more entities (by UUID) from the selection.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SelectRemove {
            /// Which entities to unselect
            pub entities: Vec<Uuid>,
        }

        /// Removes all of the Objects in the scene
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SceneClearAll;

        /// Replaces current selection with these entities (by UUID).
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SelectReplace {
            /// Which entities to select
            pub entities: Vec<Uuid>,
        }

        /// Changes the current highlighted entity to whichever one is at the given window coordinate.
        /// If there's no entity at this location, clears the highlight.
        #[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct HighlightSetEntity {
            /// Coordinates of the window being clicked
            pub selected_at_window: Point2d,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Changes the current highlighted entity to these entities.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct HighlightSetEntities {
            /// Highlight these entities.
            pub entities: Vec<Uuid>,
        }

        /// Create a new annotation
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct NewAnnotation {
            /// What should the annotation contain?
            pub options: AnnotationOptions,
            /// If true, any existing drawables within the obj will be replaced (the object will be reset)
            pub clobber: bool,
            /// What type of annotation to create.
            pub annotation_type: AnnotationType,
        }

        /// Update an annotation
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct UpdateAnnotation {
            /// Which annotation to update
            pub annotation_id: Uuid,
            /// If any of these fields are set, they will overwrite the previous options for the
            /// annotation.
            pub options: AnnotationOptions,
        }

        /// Hide or show an object
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct ObjectVisible {
            /// Which object to change
            pub object_id: Uuid,
            /// Whether or not the object should be hidden.
            pub hidden: bool,
        }

        /// Bring an object to the front of the scene
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct ObjectBringToFront {
            /// Which object to change
            pub object_id: Uuid,
        }

        /// Set the material properties of an object
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct ObjectSetMaterialParamsPbr {
            /// Which object to change
            pub object_id: Uuid,
            /// Color of the new material
            pub color: Color,
            /// Metalness of the new material
            pub metalness: f32,
            /// Roughness of the new material
            pub roughness: f32,
            /// Ambient Occlusion of the new material
            pub ambient_occlusion: f32,
        }
        /// What type of entity is this?
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct GetEntityType {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// Gets all faces which use the given edge.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetAllEdgeFaces {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the faces of.
            pub edge_id: Uuid,
        }

        /// Add a hole to a Solid2d object before extruding it.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct Solid2dAddHole {
            /// Which object to add the hole to.
            pub object_id: Uuid,
            /// The id of the path to use as the inner profile (hole).
            pub hole_id: Uuid,
        }

        /// Gets all edges which are opposite the given edge, across all possible faces.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetAllOppositeEdges {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposites of.
            pub edge_id: Uuid,
            /// If given, only faces parallel to this vector will be considered.
            pub along_vector: Option<Point3d<f64>>,
        }

        /// Gets the edge opposite the given edge, along the given face.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetOppositeEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Gets the next adjacent edge for the given edge, along the given face.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetNextAdjacentEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Gets the previous adjacent edge for the given edge, along the given face.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetPrevAdjacentEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Fillets the given edge with the specified radius.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct Solid3dFilletEdge {
            /// Which object is being filletted.
            pub object_id: Uuid,
            /// Which edge you want to fillet.
            pub edge_id: Uuid,
            /// The radius of the fillet. Measured in length (using the same units that the current sketch uses). Must be positive (i.e. greater than zero).
            pub radius: LengthUnit,
            /// The maximum acceptable surface gap computed between the filleted surfaces. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
        }

        /// Determines whether a brep face is planar and returns its surface-local planar axes if so
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct FaceIsPlanar {
            /// Which face is being queried.
            pub object_id: Uuid,
        }

        /// Determines a position on a brep face evaluated by parameters u,v
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct FaceGetPosition {
            /// Which face is being queried.
            pub object_id: Uuid,

            /// The 2D paramter-space u,v position to evaluate the surface at
            pub uv: Point2d<f64>,
        }

        /// Determines the gradient (dFdu, dFdv) + normal vector on a brep face evaluated by parameters u,v
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct FaceGetGradient {
            /// Which face is being queried.
            pub object_id: Uuid,

            /// The 2D paramter-space u,v position to evaluate the surface at
            pub uv: Point2d<f64>,
        }

        /// Send object to front or back.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SendObject {
            /// Which object is being changed.
            pub object_id: Uuid,
            /// Bring to front = true, send to back = false.
            pub front: bool,
        }
        /// Set opacity of the entity.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct EntitySetOpacity {
            /// Which entity is being changed.
            pub entity_id: Uuid,
            /// How transparent should it be?
            /// 0 or lower is totally transparent.
            /// 1 or greater is totally opaque.
            pub opacity: f32,
        }

        /// Fade entity in or out.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct EntityFade {
            /// Which entity is being changed.
            pub entity_id: Uuid,
            /// Fade in = true, fade out = false.
            pub fade_in: bool,
            /// How many seconds the animation should take.
            #[serde(default = "default_animation_seconds")]
            pub duration_seconds: f32,
        }

        /// Make a new plane
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct MakePlane {
            /// Origin of the plane
            pub origin: Point3d<LengthUnit>,
            /// What should the plane's X axis be?
            pub x_axis: Point3d<f64>,
            /// What should the plane's Y axis be?
            pub y_axis: Point3d<f64>,
            /// What should the plane's span/extent?
            /// When rendered visually, this is both the
            /// width and height along X and Y axis respectively.
            pub size: LengthUnit,
            /// If true, any existing drawables within the obj will be replaced (the object will be reset)
            pub clobber: bool,
            /// If true, the plane will be created but hidden initially.
            pub hide: Option<bool>,
        }

        /// Set the color of a plane.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct PlaneSetColor {
            /// Which plane is being changed.
            pub plane_id: Uuid,
            /// What color it should be.
            pub color: Color,
        }

        /// Set the current tool.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SetTool {
            /// What tool should be active.
            pub tool: SceneToolType,
        }

        /// Send a mouse move event
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct MouseMove {
            /// Where the mouse is
            pub window: Point2d,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Send a mouse click event
        /// Updates modified/selected entities.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct MouseClick {
            /// Where the mouse is
            pub window: Point2d,
        }

        /// Enable sketch mode on the given plane.
        /// If you want to sketch on a face, use `enable_sketch_mode` instead.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SketchModeEnable {
            /// Sketch on this plane.
            pub plane_id: Uuid,
            /// Use an orthographic camera.
            pub ortho: bool,
            /// Animate the transition to sketch mode.
            pub animated: bool,
            /// Disable the camera entirely for sketch mode and sketch on a plane (this would be the normal
            /// of that plane).
            pub disable_camera_with_plane: Option<Point3d<f64>>,
        }

        /// Disable sketch mode.
        /// If you are sketching on a face, be sure to not disable sketch mode until you have extruded.
        /// Otherwise, your object will not be fused with the face.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SketchModeDisable;

        /// Get the plane for sketch mode.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct GetSketchModePlane;

        /// Get the plane for sketch mode.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct CurveSetConstraint {
            /// Which curve to constrain.
            pub object_id: Uuid,
            /// Which constraint to apply.
            pub constraint_bound: PathComponentConstraintBound,
            /// What part of the curve should be constrained.
            pub constraint_type: PathComponentConstraintType,
        }

        /// Sketch on some entity (e.g. a plane, a face).
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct EnableSketchMode {
            /// Which entity to sketch on.
            pub entity_id: Uuid,
            /// Should the camera use orthographic projection?
            /// In other words, should an object's size in the rendered image stay constant regardless of its distance from the camera.
            pub ortho: bool,
            /// Should we animate or snap for the camera transition?
            pub animated: bool,
            /// Should the camera move at all?
            pub adjust_camera: bool,
        }

        /// Set the background color of the scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SetBackgroundColor {
            /// The color to set the background to.
            pub color: Color,
        }

        /// Get type of the given curve.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CurveGetType {
            /// Which curve to query.
            pub curve_id: Uuid,
        }

        /// Get control points of the given curve.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CurveGetControlPoints {
            /// Which curve to query.
            pub curve_id: Uuid,
        }

        /// Enum containing the variety of image formats snapshots may be exported to.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FromStr, Display, ExecutionPlanValue)]
        #[serde(rename_all = "snake_case")]
        #[display(style = "snake_case")]
        pub enum ImageFormat {
            /// .png format
            Png,
            /// .jpeg format
            Jpeg,
        }

        /// Take a snapshot of the current view.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct TakeSnapshot {
            /// What image format to return.
            pub format: ImageFormat,
        }

        /// Add a gizmo showing the axes.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct MakeAxesGizmo {
            /// If true, axes gizmo will be placed in the corner of the screen.
            /// If false, it will be placed at the origin of the scene.
            pub gizmo_mode: bool,
            /// If true, any existing drawables within the obj will be replaced (the object will be reset)
            pub clobber: bool,
        }

        /// Query the given path.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct PathGetInfo {
            /// Which path to query
            pub path_id: Uuid,
        }

        /// Obtain curve ids for vertex ids
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct PathGetCurveUuidsForVertices {
            /// Which path to query
            pub path_id: Uuid,

            /// IDs of the vertices for which to obtain curve ids from
            pub vertex_ids: Vec<Uuid>,
        }

        /// Obtain vertex ids for a path
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct PathGetVertexUuids {
            /// Which path to query
            pub path_id: Uuid,
        }

        /// Start dragging the mouse.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct HandleMouseDragStart {
            /// The mouse position.
            pub window: Point2d,
        }

        /// Continue dragging the mouse.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct HandleMouseDragMove {
            /// The mouse position.
            pub window: Point2d,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Stop dragging the mouse.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct HandleMouseDragEnd {
            /// The mouse position.
            pub window: Point2d,
        }

        /// Remove scene objects.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty, ExecutionPlanValue)]
        pub struct RemoveSceneObjects {
            /// Objects to remove.
            pub object_ids: HashSet<Uuid>,
        }

        /// Utility method. Performs both a ray cast and projection to plane-local coordinates.
        /// Returns the plane coordinates for the given window coordinates.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct PlaneIntersectAndProject {
            /// The plane you're intersecting against.
            pub plane_id: Uuid,
            /// Window coordinates where the ray cast should be aimed.
            pub window: Point2d,
        }

        /// Find the start and end of a curve.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CurveGetEndPoints {
            /// ID of the curve being queried.
            pub curve_id: Uuid,
        }

        /// Reconfigure the stream.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct ReconfigureStream {
            /// Width of the stream.
            pub width: u32,
            /// Height of the stream.
            pub height: u32,
            /// Frames per second.
            pub fps: u32,
        }

        /// Import files to the current model.
        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ExecutionPlanValue, ModelingCmdVariant)]
        pub struct ImportFiles {
            /// Files to import.
            pub files: Vec<super::ImportFile>,
            /// Input file format.
            pub format: crate::format::InputFormat,
        }

        /// Set the units of the scene.
        /// For all following commands, the units will be interpreted as the given units.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariantEmpty)]
        pub struct SetSceneUnits {
            /// Which units the scene uses.
            pub unit: units::UnitLength,
        }

        /// Get the mass of entities in the scene or the default scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Mass {
            /// IDs of the entities to get the mass of. If this is empty, then the default scene is included in
            /// the mass.
            pub entity_ids: Vec<Uuid>,
            /// The material density.
            pub material_density: f64,
            /// The material density unit.
            pub material_density_unit: units::UnitDensity,
            /// The output unit for the mass.
            pub output_unit: units::UnitMass,
        }

        /// Get the density of entities in the scene or the default scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Density {
            /// IDs of the entities to get the density of. If this is empty, then the default scene is included in
            /// the density.
            pub entity_ids: Vec<Uuid>,
            /// The material mass.
            pub material_mass: f64,
            /// The material mass unit.
            pub material_mass_unit: units::UnitMass,
            /// The output unit for the density.
            pub output_unit: units::UnitDensity,
        }

        /// Get the volume of entities in the scene or the default scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Volume {
            /// IDs of the entities to get the volume of. If this is empty, then the default scene is included in
            /// the volume.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the volume.
            pub output_unit: units::UnitVolume,
        }

        /// Get the center of mass of entities in the scene or the default scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct CenterOfMass {
            /// IDs of the entities to get the center of mass of. If this is empty, then the default scene is included in
            /// the center of mass.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the center of mass.
            pub output_unit: units::UnitLength,
        }

        /// Get the surface area of entities in the scene or the default scene.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct SurfaceArea {
            /// IDs of the entities to get the surface area of. If this is empty, then the default scene is included in
            /// the surface area.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the surface area.
            pub output_unit: units::UnitArea,
        }

        /// Focus the default camera upon an object in the scene.
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct DefaultCameraFocusOn {
            /// UUID of object to focus on.
            pub uuid: Uuid,
        }
        /// When you select some entity with the current tool, what should happen to the entity?
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct SetSelectionType {
            /// What type of selection should occur when you select something?
            pub selection_type: SceneSelectionType,
        }

        /// What kind of entities can be selected?
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct SetSelectionFilter {
            /// If vector is empty, clear all filters.
            /// If vector is non-empty, only the given entity types will be selectable.
            pub filter: Vec<EntityType>,
        }

        /// Use orthographic projection.
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct DefaultCameraSetOrthographic;

        /// Use perspective projection.
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct DefaultCameraSetPerspective {
            /// If this is not given, use the same parameters as last time the perspective camera was used.
            pub parameters: Option<PerspectiveCameraParameters>,
        }

        /// Get a concise description of all of an extrusion's faces.
        #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct Solid3dGetExtrusionFaceInfo {
            /// The Solid3d object whose extrusion is being queried.
            pub object_id: Uuid,
            /// Any edge that lies on the extrusion base path.
            pub edge_id: Uuid,
        }

        /// Exit edit mode
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct EditModeExit;

        /// Clear the selection
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariantEmpty,
        )]
        pub struct SelectClear;

        /// Find all IDs of selected entities
        #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariant)]
        pub struct SelectGet;

        /// Get the number of objects in the scene
        #[derive(
            Clone, Debug, Deserialize, JsonSchema, Serialize, ExecutionPlanFromMemory, ModelingCmdVariant,
        )]
        pub struct GetNumObjects;
    }
}

impl ModelingCmd {
    /// Is this command safe to run in an engine batch?
    pub fn is_safe_to_batch(&self) -> bool {
        use ModelingCmd::*;
        matches!(
            self,
            MovePathPen(_)
                | ExtendPath(_)
                | Extrude(_)
                | Revolve(_)
                | Solid3dFilletEdge(_)
                | ClosePath(_)
                | UpdateAnnotation(_)
                | ObjectVisible(_)
                | ObjectBringToFront(_)
                | Solid2dAddHole(_)
                | SendObject(_)
                | EntitySetOpacity(_)
                | PlaneSetColor(_)
                | SetTool(_)
        )
    }
}

/// File to import into the current model.
/// If you are sending binary data for a file, be sure to send the WebSocketRequest as
/// binary/bson, not text/json.
#[derive(
    Debug, Clone, Serialize, Deserialize, JsonSchema, ExecutionPlanValue, ExecutionPlanFromMemory, Eq, PartialEq,
)]
pub struct ImportFile {
    /// The file's full path, including file extension.
    pub path: String,
    /// The raw bytes of the file
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub data: Vec<u8>,
}
