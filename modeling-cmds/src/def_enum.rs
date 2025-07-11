use kittycad_modeling_cmds_macros::define_modeling_cmd_enum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use self::each_cmd::*;
use crate::{self as kittycad_modeling_cmds};

define_modeling_cmd_enum! {
    pub mod each_cmd {
        use std::collections::HashSet;

        use crate::{self as kittycad_modeling_cmds};
        use kittycad_modeling_cmds_macros::{ModelingCmdVariant};
        use parse_display_derive::{Display, FromStr};
        use schemars::JsonSchema;
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;
        use crate::shared::CameraViewState;

        use crate::{
            format::{OutputFormat2d, OutputFormat3d},
            id::ModelingCmdId,
            length_unit::LengthUnit,
            shared::{
                Angle,
                ComponentTransform,
                RelativeTo,
                CutType,
                CutStrategy,
                CameraMovement,
                ExtrudedFaceInfo, ExtrudeMethod,
                AnnotationOptions, AnnotationType, CameraDragInteractionType, Color, DistanceType, EntityType,
                PathComponentConstraintBound, PathComponentConstraintType, PathSegment, PerspectiveCameraParameters,
                Point2d, Point3d, SceneSelectionType, SceneToolType, Opposite,
            },
            units,
        };

        /// Mike says this usually looks nice.
        fn default_animation_seconds() -> f64 {
            0.4
        }

        /// Default empty uuid vector.
        fn default_uuid_vector() -> Vec<Uuid> {
            Vec::new()
        }

        /// Evaluates the position of a path in one shot (engine utility for kcl executor)
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EngineUtilEvaluatePath {
            /// The path in json form (the serialized result of the kcl Sketch/Path object
            pub path_json: String,

            /// The evaluation parameter (path curve parameter in the normalized domain [0, 1])
            pub t: f64,
        }

        /// Start a new path.
        #[derive(
            Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct StartPath {}

        /// Move the path's "pen".
        /// If you're in sketch mode, these coordinates are in the local coordinate system,
        /// not the world's coordinate system.
        /// For example, say you're sketching on the plane {x: (1,0,0), y: (0,1,0), origin: (0, 0, 50)}.
        /// In other words, the plane 50 units above the default XY plane. Then, moving the pen
        /// to (1, 1, 0) with this command uses local coordinates. So, it would move the pen to
        /// (1, 1, 50) in global coordinates.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct MovePathPen {
            /// The ID of the command which created the path.
            pub path: ModelingCmdId,
            /// Where the path's pen should be.
            pub to: Point3d<LengthUnit>,
        }

        /// Extend a path by adding a new segment which starts at the path's "pen".
        /// If no "pen" location has been set before (via `MovePen`), then the pen is at the origin.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ExtendPath {
            /// The ID of the command which created the path.
            pub path: ModelingCmdId,
            /// Segment to append to the path.
            /// This segment will implicitly begin at the current "pen" location.
            pub segment: PathSegment,
        }


        /// Command for extruding a solid 2d.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Extrude {
            /// Which sketch to extrude.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// How far off the plane to extrude
            pub distance: LengthUnit,
            /// Which IDs should the new faces have?
            /// If this isn't given, the engine will generate IDs.
            #[serde(default)]
            pub faces: Option<ExtrudedFaceInfo>,
            /// Should the extrusion also extrude in the opposite direction?
            /// If so, this specifies its distance.
            #[serde(default)]
            pub opposite: Opposite<LengthUnit>,
            /// Should the extrusion create a new object or be part of the existing object.
            #[serde(default)]
            pub extrude_method: ExtrudeMethod,
        }

        fn default_twist_extrude_section_interval() -> Angle {
            Angle::from_degrees(15.0)
        }

        /// Command for twist extruding a solid 2d.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct TwistExtrude {
            /// Which sketch to extrude.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// How far off the plane to extrude
            pub distance: LengthUnit,
            /// Which IDs should the new faces have?
            /// If this isn't given, the engine will generate IDs.
            #[serde(default)]
            pub faces: Option<ExtrudedFaceInfo>,
            /// Center to twist about (relative to 2D sketch)
            #[serde(default)]
            pub center_2d: Point2d<f64>,
            /// Total rotation of the section
            pub total_rotation_angle: Angle,
            ///Angle step interval (converted to whole number degrees and bounded between 4° and 90°)
            #[serde(default = "default_twist_extrude_section_interval")]
            pub angle_step_size: Angle,
            ///The twisted surface loft tolerance
            pub tolerance: LengthUnit,
        }

        /// Extrude the object along a path.
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Sweep {
            /// Which sketch to sweep.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// Path along which to sweep.
            pub trajectory: ModelingCmdId,
            /// If true, the sweep will be broken up into sub-sweeps (extrusions, revolves, sweeps) based on the trajectory path components.
            pub sectional: bool,
            /// The maximum acceptable surface gap computed between the revolution surface joints. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
            /// What is this sweep relative to?
            #[serde(default)]
            pub relative_to: RelativeTo,
        }

        /// Command for revolving a solid 2d.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Revolve {
            /// Which sketch to revolve.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// The origin of the extrusion axis
            pub origin: Point3d<LengthUnit>,
            /// The axis of the extrusion (taken from the origin)
            pub axis: Point3d<f64>,
            /// If true, the axis is interpreted within the 2D space of the solid 2D's plane
            pub axis_is_2d: bool,
            /// The signed angle of revolution (in degrees, must be <= 360 in either direction)
            pub angle: Angle,
            /// The maximum acceptable surface gap computed between the revolution surface joints. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
            /// Should the revolution also revolve in the opposite direction along the given axis?
            /// If so, this specifies its angle.
            #[serde(default)]
            pub opposite: Opposite<Angle>,
        }

        /// Command for shelling a solid3d face
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dShellFace {
            /// Which Solid3D is being shelled.
            pub object_id: Uuid,
            /// Which faces to remove, leaving only the shell.
            pub face_ids: Vec<Uuid>,
            /// How thick the shell should be.
            /// Smaller values mean a thinner shell.
            pub shell_thickness: LengthUnit,
            /// If true, the Solid3D is made hollow instead of removing the selected faces
            #[serde(default)]
            pub hollow: bool,
        }

        /// Command for revolving a solid 2d about a brep edge
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct RevolveAboutEdge {
            /// Which sketch to revolve.
            /// Must be a closed 2D solid.
            pub target: ModelingCmdId,
            /// The edge to use as the axis of revolution, must be linear and lie in the plane of the solid
            pub edge_id: Uuid,
            /// The signed angle of revolution (in degrees, must be <= 360 in either direction)
            pub angle: Angle,
            /// The maximum acceptable surface gap computed between the revolution surface joints. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
            /// Should the revolution also revolve in the opposite direction along the given axis?
            /// If so, this specifies its angle.
            #[serde(default)]
            pub opposite: Opposite<Angle>,
        }

        /// Command for lofting sections to create a solid
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Loft {
            /// The closed section curves to create a lofted solid from.
            /// Currently, these must be Solid2Ds
            pub section_ids: Vec<Uuid>,
            /// Degree of the interpolation. Must be greater than zero.
            /// For example, use 2 for quadratic, or 3 for cubic interpolation in the V direction.
            pub v_degree: std::num::NonZeroU32,
            /// Attempt to approximate rational curves (such as arcs) using a bezier.
            /// This will remove banding around interpolations between arcs and non-arcs.  It may produce errors in other scenarios
            /// Over time, this field won't be necessary.
            pub bez_approximate_rational: bool,
            /// This can be set to override the automatically determined topological base curve, which is usually the first section encountered.
            pub base_curve_index: Option<u32>,
            /// Tolerance
            pub tolerance: LengthUnit,
        }


        /// Closes a path, converting it to a 2D solid.
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ClosePath {
            /// Which path to close.
            pub path_id: Uuid,
        }

        /// Camera drag started.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CameraDragStart {
            /// The type of camera drag interaction.
            pub interaction: CameraDragInteractionType,
            /// The initial mouse position.
            pub window: Point2d,
        }

        /// Camera drag continued.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CameraDragEnd {
            /// The type of camera drag interaction.
            pub interaction: CameraDragInteractionType,
            /// The final mouse position.
            pub window: Point2d,
        }

        /// Gets the default camera's camera settings
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraGetSettings {}

        /// Gets the default camera's view state
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraGetView {}

        /// Sets the default camera's view state
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraSetView {
            /// Camera view state
            pub view: CameraViewState,
        }

        /// Change what the default camera is looking at.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraPerspectiveSettings {
            /// Where the camera is positioned
            pub vantage: Point3d,
            /// What the camera is looking at. Center of the camera's field of vision
            pub center: Point3d,
            /// Which way is "up", from the camera's point of view.
            pub up: Point3d,
            /// The field of view angle in the y direction, in degrees.
            pub fov_y: Option<f32>,
            /// The distance to the near clipping plane.
            pub z_near: Option<f32>,
            /// The distance to the far clipping plane.
            pub z_far: Option<f32>,
            /// Logical timestamp. The client should increment this
            /// with every event in the current mouse drag. That way, if the
            /// events are being sent over an unordered channel, the API
            /// can ignore the older events.
            pub sequence: Option<u32>,
        }

        /// Adjust zoom of the default camera.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraZoom {
            /// Move the camera forward along the vector it's looking at,
            /// by this magnitudedefaultCameraZoom.
            /// Basically, how much should the camera move forward by.
            pub magnitude: f32,
        }

        /// Export a sketch to a file.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Export2d {
            /// IDs of the entities to be exported.
            pub entity_ids: Vec<Uuid>,
            /// The file format to export to.
            pub format: OutputFormat2d,
        }

        /// Export the scene to a file.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Export3d {
            /// IDs of the entities to be exported. If this is empty, then all entities are exported.
            pub entity_ids: Vec<Uuid>,
            /// The file format to export to.
            pub format: OutputFormat3d,
        }

        /// Export the scene to a file.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Export {
            /// IDs of the entities to be exported. If this is empty, then all entities are exported.
            pub entity_ids: Vec<Uuid>,
            /// The file format to export to.
            pub format: OutputFormat3d,
        }

        /// What is this entity's parent?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetParentId {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// How many children does the entity have?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetNumChildren {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// What is the UUID of this entity's n-th child?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetChildUuid {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
            /// Index into the entity's list of children.
            pub child_index: u32,
        }

        /// What are all UUIDs of this entity's children?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetAllChildUuids {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// What are all UUIDs of all the paths sketched on top of this entity?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetSketchPaths {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// What is the distance between these two entities?
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityGetDistance {
            /// ID of the first entity being queried.
            pub entity_id1: Uuid,
            /// ID of the second entity being queried.
            pub entity_id2: Uuid,
            /// Type of distance to be measured.
            pub distance_type: DistanceType,
        }

        /// Create a pattern using this entity by specifying the transform for each desired repetition.
        /// Transformations are performed in the following order (first applied to last applied): scale, rotate, translate.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityClone {
            /// ID of the entity being cloned.
            pub entity_id: Uuid,
        }

        /// Create a pattern using this entity by specifying the transform for each desired repetition.
        /// Transformations are performed in the following order (first applied to last applied): scale, rotate, translate.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityLinearPatternTransform {
            /// ID of the entity being copied.
            pub entity_id: Uuid,
            /// How to transform each repeated solid.
            /// The 0th transform will create the first copy of the entity.
            /// The total number of (optional) repetitions equals the size of this list.
            #[serde(default)]
            pub transform: Vec<crate::shared::Transform>,
            /// Alternatively, you could set this key instead.
            /// If you want to use multiple transforms per item.
            /// If this is non-empty then the `transform` key must be empty, and vice-versa.
            #[serde(default)]
            pub transforms: Vec<Vec<crate::shared::Transform>>,
        }

        /// Create a linear pattern using this entity.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityMakeHelix {
            /// ID of the cylinder.
            pub cylinder_id: Uuid,
            /// Number of revolutions.
            pub revolutions: f64,
            /// Start angle.
            #[serde(default)]
            pub start_angle: Angle,
            /// Is the helix rotation clockwise?
            pub is_clockwise: bool,
            /// Length of the helix.
            pub length: LengthUnit,
        }

        /// Create a helix using the specified parameters.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityMakeHelixFromParams {
            /// Radius of the helix.
            pub radius: LengthUnit,
            /// Length of the helix.
            pub length: LengthUnit,
            /// Number of revolutions.
            pub revolutions: f64,
            /// Start angle.
            #[serde(default)]
            pub start_angle: Angle,
            /// Is the helix rotation clockwise?
            pub is_clockwise: bool,
            /// Center of the helix at the base of the helix.
            pub center: Point3d<LengthUnit>,
            /// Axis of the helix. The helix will be created around and in the direction of this axis.
            pub axis: Point3d<f64>,
        }

        /// Create a helix using the specified parameters.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityMakeHelixFromEdge {
            /// Radius of the helix.
            pub radius: LengthUnit,
            /// Length of the helix. If None, the length of the edge will be used instead.
            pub length: Option<LengthUnit>,
            /// Number of revolutions.
            pub revolutions: f64,
            /// Start angle.
            #[serde(default)]
            pub start_angle: Angle,
            /// Is the helix rotation clockwise?
            pub is_clockwise: bool,
            /// Edge about which to make the helix.
            pub edge_id: Uuid,
        }

        /// Mirror the input entities over the specified axis. (Currently only supports sketches)
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityMirror {
            /// ID of the mirror entities.
            pub ids: Vec<Uuid>,
            /// Axis to use as mirror.
            pub axis: Point3d<f64>,
            /// Point through which the mirror axis passes.
            pub point: Point3d<LengthUnit>,
        }

        /// Mirror the input entities over the specified edge. (Currently only supports sketches)
        #[derive(
            Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityMirrorAcrossEdge {
            /// ID of the mirror entities.
            pub ids: Vec<Uuid>,
            /// The edge to use as the mirror axis, must be linear and lie in the plane of the solid
            pub edge_id: Uuid,
        }

        /// Modifies the selection by simulating a "mouse click" at the given x,y window coordinate
        /// Returns ID of whatever was selected.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectWithPoint {
            /// Where in the window was selected
            pub selected_at_window: Point2d,
            /// What entity was selected?
            pub selection_type: SceneSelectionType,
        }

        /// Adds one or more entities (by UUID) to the selection.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectAdd {
            /// Which entities to select
            pub entities: Vec<Uuid>,
        }

        /// Removes one or more entities (by UUID) from the selection.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectRemove {
            /// Which entities to unselect
            pub entities: Vec<Uuid>,
        }

        /// Removes all of the Objects in the scene
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SceneClearAll {}

        /// Replaces current selection with these entities (by UUID).
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectReplace {
            /// Which entities to select
            pub entities: Vec<Uuid>,
        }

        /// Changes the current highlighted entity to whichever one is at the given window coordinate.
        /// If there's no entity at this location, clears the highlight.
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct HighlightSetEntities {
            /// Highlight these entities.
            pub entities: Vec<Uuid>,
        }

        /// Create a new annotation
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct NewAnnotation {
            /// What should the annotation contain?
            pub options: AnnotationOptions,
            /// If true, any existing drawables within the obj will be replaced (the object will be reset)
            pub clobber: bool,
            /// What type of annotation to create.
            pub annotation_type: AnnotationType,
        }

        /// Update an annotation
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct UpdateAnnotation {
            /// Which annotation to update
            pub annotation_id: Uuid,
            /// If any of these fields are set, they will overwrite the previous options for the
            /// annotation.
            pub options: AnnotationOptions,
        }

        /// Changes visibility of scene-wide edge lines on brep solids
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EdgeLinesVisible {
            /// Whether or not the edge lines should be hidden.
            pub hidden: bool,
        }

        /// Hide or show an object
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ObjectVisible {
            /// Which object to change
            pub object_id: Uuid,
            /// Whether or not the object should be hidden.
            pub hidden: bool,
        }

        /// Bring an object to the front of the scene
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ObjectBringToFront {
            /// Which object to change
            pub object_id: Uuid,
        }

        /// Set the material properties of an object
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct GetEntityType {
            /// ID of the entity being queried.
            pub entity_id: Uuid,
        }

        /// Gets all faces which use the given edge.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetAllEdgeFaces {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the faces of.
            pub edge_id: Uuid,
        }

        /// Add a hole to a Solid2d object before extruding it.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid2dAddHole {
            /// Which object to add the hole to.
            pub object_id: Uuid,
            /// The id of the path to use as the inner profile (hole).
            pub hole_id: Uuid,
        }

        /// Gets all edges which are opposite the given edge, across all possible faces.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetAllOppositeEdges {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposites of.
            pub edge_id: Uuid,
            /// If given, only faces parallel to this vector will be considered.
            pub along_vector: Option<Point3d<f64>>,
        }

        /// Gets the edge opposite the given edge, along the given face.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetOppositeEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Gets the next adjacent edge for the given edge, along the given face.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetNextAdjacentEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Gets the previous adjacent edge for the given edge, along the given face.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetPrevAdjacentEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// Which edge you want the opposite of.
            pub edge_id: Uuid,
            /// Which face is used to figure out the opposite edge?
            pub face_id: Uuid,
        }

        /// Gets the shared edge between these two faces if it exists
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetCommonEdge {
            /// Which object is being queried.
            pub object_id: Uuid,
            /// The faces being queried
            pub face_ids: [Uuid; 2]
        }

        /// Fillets the given edge with the specified radius.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dFilletEdge {
            /// Which object is being filletted.
            pub object_id: Uuid,
            /// Which edge you want to fillet.
            #[serde(default)]
            pub edge_id: Option<Uuid>,
            /// Which edges you want to fillet.
            #[serde(default)]
            pub edge_ids: Vec<Uuid>,
            /// The radius of the fillet. Measured in length (using the same units that the current sketch uses). Must be positive (i.e. greater than zero).
            pub radius: LengthUnit,
            /// The maximum acceptable surface gap computed between the filleted surfaces. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
            /// How to apply the cut.
            #[serde(default)]
            pub cut_type: CutType,
            /// Which cutting algorithm to use.
            #[serde(default)]
            pub strategy: CutStrategy,
            /// What IDs should the resulting faces have?
            /// If you've only passed one edge ID, its ID will
            /// be the command ID used to send this command, and this
            /// field should be empty.
            /// If you've passed `n` IDs (to fillet `n` edges), then
            /// this should be length `n-1`, and the first edge will use
            /// the command ID used to send this command.
            #[serde(default)]
            pub extra_face_ids: Vec<Uuid>,
        }

        /// Determines whether a brep face is planar and returns its surface-local planar axes if so
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct FaceIsPlanar {
            /// Which face is being queried.
            pub object_id: Uuid,
        }

        /// Determines a position on a brep face evaluated by parameters u,v
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct FaceGetPosition {
            /// Which face is being queried.
            pub object_id: Uuid,

            /// The 2D parameter-space u,v position to evaluate the surface at
            pub uv: Point2d<f64>,
        }

        ///Obtains the surface "center of mass"
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct FaceGetCenter {
            /// Which face is being queried.
            pub object_id: Uuid,
        }

        /// Determines the gradient (dFdu, dFdv) + normal vector on a brep face evaluated by parameters u,v
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct FaceGetGradient {
            /// Which face is being queried.
            pub object_id: Uuid,

            /// The 2D parameter-space u,v position to evaluate the surface at
            pub uv: Point2d<f64>,
        }

        /// Send object to front or back.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SendObject {
            /// Which object is being changed.
            pub object_id: Uuid,
            /// Bring to front = true, send to back = false.
            pub front: bool,
        }
        /// Set opacity of the entity.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntitySetOpacity {
            /// Which entity is being changed.
            pub entity_id: Uuid,
            /// How transparent should it be?
            /// 0 or lower is totally transparent.
            /// 1 or greater is totally opaque.
            pub opacity: f32,
        }

        /// Fade entity in or out.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EntityFade {
            /// Which entity is being changed.
            pub entity_id: Uuid,
            /// Fade in = true, fade out = false.
            pub fade_in: bool,
            /// How many seconds the animation should take.
            #[serde(default = "default_animation_seconds")]
            pub duration_seconds: f64,
        }

        /// Make a new plane
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PlaneSetColor {
            /// Which plane is being changed.
            pub plane_id: Uuid,
            /// What color it should be.
            pub color: Color,
        }

        /// Set the current tool.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetTool {
            /// What tool should be active.
            pub tool: SceneToolType,
        }

        /// Send a mouse move event
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct MouseClick {
            /// Where the mouse is
            pub window: Point2d,
        }

        /// Disable sketch mode.
        /// If you are sketching on a face, be sure to not disable sketch mode until you have extruded.
        /// Otherwise, your object will not be fused with the face.
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SketchModeDisable {}

        /// Get the plane for sketch mode.
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct GetSketchModePlane {}

        /// Get the plane for sketch mode.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CurveSetConstraint {
            /// Which curve to constrain.
            pub object_id: Uuid,
            /// Which constraint to apply.
            pub constraint_bound: PathComponentConstraintBound,
            /// What part of the curve should be constrained.
            pub constraint_type: PathComponentConstraintType,
        }

        /// Sketch on some entity (e.g. a plane, a face).
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
            /// If provided, ensures that the normal of the sketch plane must be aligned with this supplied normal
            /// (otherwise the camera position will be used to infer the normal to point towards the viewer)
            pub planar_normal: Option<Point3d<f64>>,
        }

        /// Sets whether or not changes to the scene or its objects will be done as a "dry run"
        /// In a dry run, successful commands won't actually change the model.
        /// This is useful for catching errors before actually making the change.
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct EnableDryRun {}

        /// Sets whether or not changes to the scene or its objects will be done as a "dry run"
        /// In a dry run, successful commands won't actually change the model.
        /// This is useful for catching errors before actually making the change.
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DisableDryRun {}

        /// Set the background color of the scene.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetBackgroundColor {
            /// The color to set the background to.
            pub color: Color,
        }

        /// Set the properties of the tool lines for the scene.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetCurrentToolProperties {
            /// The color to set the tool line to.
            pub color: Option<Color>,
        }

        /// Set the default system properties used when a specific property isn't set.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetDefaultSystemProperties {
            /// The default system color.
            pub color: Option<Color>,
        }

        /// Get type of the given curve.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CurveGetType {
            /// Which curve to query.
            pub curve_id: Uuid,
        }

        /// Get control points of the given curve.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CurveGetControlPoints {
            /// Which curve to query.
            pub curve_id: Uuid,
        }

        /// Project an entity on to a plane.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ProjectEntityToPlane {
            /// Which entity to project (vertex or edge).
            pub entity_id: Uuid,
            /// Which plane to project entity_id onto.
            pub plane_id: Uuid,
            /// If true: the projected points are returned in the plane_id's coordinate system,
            /// else: the projected points are returned in the world coordinate system.
            pub use_plane_coords: bool,
        }

        /// Project a list of points on to a plane.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ProjectPointsToPlane {
            /// The id of the plane used for the projection.
            pub plane_id: Uuid,
            /// The list of points that will be projected.
            pub points: Vec<Point3d<f64>>,
            /// If true: the projected points are returned in the plane_id's coordinate sysetm.
            /// else: the projected points are returned in the world coordinate system.
            pub use_plane_coords: bool,
        }

        /// Enum containing the variety of image formats snapshots may be exported to.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, FromStr, Display)]
        #[serde(rename_all = "snake_case")]
        #[display(style = "snake_case")]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub enum ImageFormat {
            /// .png format
            Png,
            /// .jpeg format
            Jpeg,
        }

        /// Take a snapshot of the current view.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct TakeSnapshot {
            /// What image format to return.
            pub format: ImageFormat,
        }

        /// Add a gizmo showing the axes.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct MakeAxesGizmo {
            /// If true, axes gizmo will be placed in the corner of the screen.
            /// If false, it will be placed at the origin of the scene.
            pub gizmo_mode: bool,
            /// If true, any existing drawables within the obj will be replaced (the object will be reset)
            pub clobber: bool,
        }

        /// Query the given path.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PathGetInfo {
            /// Which path to query
            pub path_id: Uuid,
        }

        /// Obtain curve ids for vertex ids
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PathGetCurveUuidsForVertices {
            /// Which path to query
            pub path_id: Uuid,

            /// IDs of the vertices for which to obtain curve ids from
            pub vertex_ids: Vec<Uuid>,
        }

        /// Obtain curve id by index
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PathGetCurveUuid {
            /// Which path to query
            pub path_id: Uuid,

            /// IDs of the vertices for which to obtain curve ids from
            pub index: u32,
        }

        /// Obtain vertex ids for a path
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PathGetVertexUuids {
            /// Which path to query
            pub path_id: Uuid,
        }

        /// Obtain the sketch target id (if the path was drawn in sketchmode) for a path
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PathGetSketchTargetUuid {
            /// Which path to query
            pub path_id: Uuid,
        }

        /// Start dragging the mouse.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct HandleMouseDragStart {
            /// The mouse position.
            pub window: Point2d,
        }

        /// Continue dragging the mouse.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct HandleMouseDragEnd {
            /// The mouse position.
            pub window: Point2d,
        }

        /// Remove scene objects.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct RemoveSceneObjects {
            /// Objects to remove.
            pub object_ids: HashSet<Uuid>,
        }

        /// Utility method. Performs both a ray cast and projection to plane-local coordinates.
        /// Returns the plane coordinates for the given window coordinates.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct PlaneIntersectAndProject {
            /// The plane you're intersecting against.
            pub plane_id: Uuid,
            /// Window coordinates where the ray cast should be aimed.
            pub window: Point2d,
        }

        /// Find the start and end of a curve.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CurveGetEndPoints {
            /// ID of the curve being queried.
            pub curve_id: Uuid,
        }

        /// Reconfigure the stream.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ReconfigureStream {
            /// Width of the stream.
            pub width: u32,
            /// Height of the stream.
            pub height: u32,
            /// Frames per second.
            pub fps: u32,
            /// Video feed's constant bitrate (CBR)
            #[serde(default)]
            pub bitrate: Option<u32>,
        }

        /// Import files to the current model.
        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ImportFiles {
            /// Files to import.
            pub files: Vec<super::ImportFile>,
            /// Input file format.
            pub format: crate::format::InputFormat3d,
        }

        /// Set the units of the scene.
        /// For all following commands, the units will be interpreted as the given units.
        /// Any previously executed commands will not be affected or have their units changed.
        /// They will remain in the units they were originally executed in.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetSceneUnits {
            /// Which units the scene uses.
            pub unit: units::UnitLength,
        }

        /// Get the mass of entities in the scene or the default scene.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Volume {
            /// IDs of the entities to get the volume of. If this is empty, then the default scene is included in
            /// the volume.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the volume.
            pub output_unit: units::UnitVolume,
        }

        /// Get the center of mass of entities in the scene or the default scene.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct CenterOfMass {
            /// IDs of the entities to get the center of mass of. If this is empty, then the default scene is included in
            /// the center of mass.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the center of mass.
            pub output_unit: units::UnitLength,
        }

        /// Get the surface area of entities in the scene or the default scene.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SurfaceArea {
            /// IDs of the entities to get the surface area of. If this is empty, then the default scene is included in
            /// the surface area.
            pub entity_ids: Vec<Uuid>,
            /// The output unit for the surface area.
            pub output_unit: units::UnitArea,
        }

        /// Focus the default camera upon an object in the scene.
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraFocusOn {
            /// UUID of object to focus on.
            pub uuid: Uuid,
        }
        /// When you select some entity with the current tool, what should happen to the entity?
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetSelectionType {
            /// What type of selection should occur when you select something?
            pub selection_type: SceneSelectionType,
        }

        /// What kind of entities can be selected?
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetSelectionFilter {
            /// If vector is empty, clear all filters.
            /// If vector is non-empty, only the given entity types will be selectable.
            pub filter: Vec<EntityType>,
        }

        /// Use orthographic projection.
        #[derive(
            Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraSetOrthographic {}

        /// Use perspective projection.
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraSetPerspective {
            /// If this is not given, use the same parameters as last time the perspective camera was used.
            pub parameters: Option<PerspectiveCameraParameters>,
        }

        ///Updates the camera to center to the center of the current selection
        ///(or the origin if nothing is selected)
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraCenterToSelection {
            /// Dictates whether or not the camera position should be adjusted during this operation
            /// If no movement is requested, the camera will orbit around the new center from its current position
            #[serde(default)]
            pub camera_movement: CameraMovement,
        }

        ///Updates the camera to center to the center of the current scene's bounds
        #[derive(
            Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct DefaultCameraCenterToScene {
            /// Dictates whether or not the camera position should be adjusted during this operation
            /// If no movement is requested, the camera will orbit around the new center from its current position
            #[serde(default)]
            pub camera_movement: CameraMovement,
        }

        /// Fit the view to the specified object(s).
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ZoomToFit {
            /// Which objects to fit camera to; if empty, fit to all non-default objects. Defaults to empty vector.
            #[serde(default = "default_uuid_vector")]
            pub object_ids: Vec<Uuid>,
            /// How much to pad the view frame by, as a fraction of the object(s) bounding box size.
            /// Negative padding will crop the view of the object proportionally.
            /// e.g. padding = 0.2 means the view will span 120% of the object(s) bounding box,
            /// and padding = -0.2 means the view will span 80% of the object(s) bounding box.
            #[serde(default)]
            pub padding: f32,
            /// Whether or not to animate the camera movement.
            #[serde(default)]
            pub animated: bool,
        }

        /// Looks along the normal of the specified face (if it is planar!), and fits the view to it.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct OrientToFace {
            /// Which face to orient camera to. If the face is not planar, no action will occur.
            pub face_id: Uuid,
            /// How much to pad the view frame by, as a fraction of the face bounding box size.
            /// Negative padding will crop the view of the face proportionally.
            /// e.g. padding = 0.2 means the view will span 120% of the face bounding box,
            /// and padding = -0.2 means the view will span 80% of the face bounding box.
            #[serde(default)]
            pub padding: f32,
            /// Whether or not to animate the camera movement. (Animation is currently not supported.)
            #[serde(default)]
            pub animated: bool,
        }

        /// Fit the view to the scene with an isometric view.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct ViewIsometric {
            /// How much to pad the view frame by, as a fraction of the object(s) bounding box size.
            /// Negative padding will crop the view of the object proportionally.
            /// e.g. padding = 0.2 means the view will span 120% of the object(s) bounding box,
            /// and padding = -0.2 means the view will span 80% of the object(s) bounding box.
            #[serde(default = "f32::default")]
            pub padding: f32,
        }

        /// Get a concise description of all of an extrusion's faces.
        #[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetExtrusionFaceInfo {
            /// The Solid3d object whose extrusion is being queried.
            pub object_id: Uuid,
            /// Any edge that lies on the extrusion base path.
            pub edge_id: Uuid,
        }

        /// Get a concise description of all of solids edges.
        #[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct Solid3dGetAdjacencyInfo {
            /// The Solid3d object whose info is being queried.
            pub object_id: Uuid,
            /// Any edge that lies on the extrusion base path.
            pub edge_id: Uuid,
        }


        /// Clear the selection
        #[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectClear {}

        /// Find all IDs of selected entities
        #[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SelectGet {}

        /// Get the number of objects in the scene
        #[derive(
            Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct GetNumObjects {}

        ///Set the transform of an object.
        #[derive(
            Clone, Debug, Deserialize, PartialEq, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetObjectTransform
        {
            /// Id of the object whose transform is to be set.
            pub object_id: Uuid,
            /// List of transforms to be applied to the object.
            pub transforms: Vec<ComponentTransform>,
        }

        /// Create a new solid from combining other smaller solids.
        /// In other words, every part of the input solids will be included in the output solid.
        #[derive(
            Clone, Debug, Deserialize, PartialEq, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct BooleanUnion
        {
            /// Which solids to union together.
            /// Cannot be empty.
            pub solid_ids: Vec<Uuid>,
            /// The maximum acceptable surface gap computed between the joined solids. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
        }

        /// Create a new solid from intersecting several other solids.
        /// In other words, the part of the input solids where they all overlap will be the output solid.
        #[derive(
            Clone, Debug, Deserialize, PartialEq, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct BooleanIntersection
        {
            /// Which solids to intersect together
            pub solid_ids: Vec<Uuid>,
            /// The maximum acceptable surface gap computed between the joined solids. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
        }

        /// Create a new solid from subtracting several other solids.
        /// The 'target' is what will be cut from.
        /// The 'tool' is what will be cut out from 'target'.
        #[derive(
            Clone, Debug, Deserialize, PartialEq, JsonSchema, Serialize, ModelingCmdVariant,
        )]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct BooleanSubtract
        {
            /// Geometry to cut out from.
            pub target_ids: Vec<Uuid>,
            /// Will be cut out from the 'target'.
            pub tool_ids: Vec<Uuid>,
            /// The maximum acceptable surface gap computed between the target and the solids cut out from it. Must be positive (i.e. greater than zero).
            pub tolerance: LengthUnit,
        }

        /// Make a new path by offsetting an object by a given distance.
        /// The new path's ID will be the ID of this command.
        #[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct MakeOffsetPath {
            /// The object that will be offset (can be a path, sketch, or a solid)
            pub object_id: Uuid,
            /// If the object is a solid, this is the ID of the face to base the offset on.
            /// If given, and `object_id` refers to a solid, then this face on the solid will be offset.
            /// If given but `object_id` doesn't refer to a solid, responds with an error.
            /// If not given, then `object_id` itself will be offset directly.
            #[serde(default)]
            pub face_id: Option<Uuid>,
            /// The distance to offset the path (positive for outset, negative for inset)
            pub offset: LengthUnit,
        }

        /// Add a hole to a closed path by offsetting it a uniform distance inward.
        #[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct AddHoleFromOffset {
            /// The closed path to add a hole to.
            pub object_id: Uuid,
            /// The distance to offset the path (positive for outset, negative for inset)
            pub offset: LengthUnit,
        }

        /// Align the grid with a plane or a planar face.
        #[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetGridReferencePlane {
            /// The grid to be moved.
            pub grid_id: Uuid,
            /// The plane or face that the grid will be aligned to.
            /// If a face, it must be planar to succeed.
            pub reference_id: Uuid,
        }

        /// Set the scale of the grid lines in the video feed.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetGridScale {
            /// Distance between grid lines represents this much distance.
            pub value: f32,
            /// Which units the `value` field uses.
            pub units: units::UnitLength,
        }
        /// Set the grid lines to auto scale. The grid will get larger the further you zoom out,
        /// and smaller the more you zoom in.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ModelingCmdVariant)]
        #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
        #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
        pub struct SetGridAutoScale {
        }
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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
