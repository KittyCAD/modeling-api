#[cfg(feature = "diesel")]
use std::str::FromStr;

#[cfg(feature = "diesel")]
use diesel::{mysql::Mysql, serialize::ToSql, sql_types::Text};
#[cfg(feature = "diesel")]
use diesel_derives::{AsExpression, FromSqlRow};
use enum_iterator::Sequence;
use kittycad_execution_plan_macros::ExecutionPlanValue;
use kittycad_execution_plan_traits as kcep;
use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cxx")]
use crate::impl_extern_type;
use crate::units::UnitAngle;

// A helper macro for allowing enums of only strings to be saved to the database.
macro_rules! impl_string_enum_sql {
    {$name:ident} => {
        #[cfg(feature = "diesel")]
        impl diesel::serialize::ToSql<Text, Mysql> for $name {
            fn to_sql<'a>(&'a self, out: &mut diesel::serialize::Output<'a, '_, Mysql>) -> diesel::serialize::Result {
                <String as ToSql<Text, Mysql>>::to_sql(&self.to_string(), &mut out.reborrow())
            }
        }

        #[cfg(feature = "diesel")]
        impl<DB> diesel::deserialize::FromSql<Text, DB> for $name
        where
            DB: diesel::backend::Backend,
            String: diesel::deserialize::FromSql<Text, DB>,
        {
            fn from_sql(bytes: <DB as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
                Ok(Self::from_str(&String::from_sql(bytes)?)?)
            }
        }
    };
}

/// Options for annotations
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnnotationOptions {
    /// Text displayed on the annotation
    pub text: Option<AnnotationTextOptions>,
    /// How to style the start and end of the line
    pub line_ends: Option<AnnotationLineEndOptions>,
    /// Width of the annotation's line
    pub line_width: Option<f32>,
    /// Color to render the annotation
    pub color: Option<Color>,
    /// Position to put the annotation
    pub position: Option<Point3d<f32>>,
}

/// Options for annotation text
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnnotationLineEndOptions {
    /// How to style the start of the annotation line.
    pub start: AnnotationLineEnd,
    /// How to style the end of the annotation line.
    pub end: AnnotationLineEnd,
}

/// Options for annotation text
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnnotationTextOptions {
    /// Alignment along the X axis
    pub x: AnnotationTextAlignmentX,
    /// Alignment along the Y axis
    pub y: AnnotationTextAlignmentY,
    /// Text displayed on the annotation
    pub text: String,
    /// Text font's point size
    pub point_size: u32,
}

/// The type of distance
/// Distances can vary depending on
/// the objects used as input.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DistanceType {
    /// Euclidean Distance.
    Euclidean {},
    /// The distance between objects along the specified axis
    OnAxis {
        /// Global axis
        axis: GlobalAxis,
    },
}

/// An RGBA color
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alpha
    pub a: f32,
}

/// Horizontal Text alignment
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum AnnotationTextAlignmentX {
    Left,
    Center,
    Right,
}

impl_string_enum_sql! {AnnotationTextAlignmentX}

/// Vertical Text alignment
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum AnnotationTextAlignmentY {
    Bottom,
    Center,
    Top,
}

impl_string_enum_sql! {AnnotationTextAlignmentY}

/// A point in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default, ExecutionPlanValue)]
#[serde(rename = "Point3d")]
#[serde(rename_all = "snake_case")]
pub struct Point3d<T = f32>
where
    kcep::Primitive: From<T>,
    T: kcep::Value,
{
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
    #[allow(missing_docs)]
    pub z: T,
}

/// Annotation line end type
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum AnnotationLineEnd {
    None,
    Arrow,
}

impl_string_enum_sql! {AnnotationLineEnd}

/// The type of annotation
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum AnnotationType {
    /// 2D annotation type (screen or planar space)
    T2D,
    /// 3D annotation type
    T3D,
}

impl_string_enum_sql! {AnnotationType}

/// The type of camera drag interaction.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum CameraDragInteractionType {
    /// Camera pan
    Pan,
    /// Camera rotate (revolve/orbit)
    Rotate,
    /// Camera zoom (increase or decrease distance to reference point center)
    Zoom,
}

/// A segment of a path.
/// Paths are composed of many segments.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PathSegment {
    /// A straight line segment.
    /// Goes from the current path "pen" to the given endpoint.
    Line {
        /// End point of the line.
        end: Point3d<f64>,
        ///Whether or not this line is a relative offset
        relative: bool,
    },
    /// A circular arc segment.
    Arc {
        /// Center of the circle
        center: Point2d<f64>,
        /// Radius of the circle
        radius: f64,
        /// Start of the arc along circle's perimeter.
        start: Angle,
        /// End of the arc along circle's perimeter.
        end: Angle,
        ///Whether or not this arc is a relative offset
        relative: bool,
    },
    /// A cubic bezier curve segment.
    /// Start at the end of the current line, go through control point 1 and 2, then end at a
    /// given point.
    Bezier {
        /// First control point.
        control1: Point3d<f64>,
        /// Second control point.
        control2: Point3d<f64>,
        /// Final control point.
        end: Point3d<f64>,
        ///Whether or not this bezier is a relative offset
        relative: bool,
    },
    /// Adds a tangent arc from current pen position with the given radius and angle.
    TangentialArc {
        /// Radius of the arc.
        /// Not to be confused with Raiders of the Lost Ark.
        radius: f64,
        /// Offset of the arc.
        offset: Angle,
    },
    /// Adds a tangent arc from current pen position to the new position.
    TangentialArcTo {
        /// Where the arc should end.
        /// Must lie in the same plane as the current path pen position.
        /// Must not be colinear with current path pen position.
        to: Point3d<f64>,
        /// 0 will be interpreted as none/null.
        angle_snap_increment: Option<Angle>,
    },
}

/// A point in homogeneous (4D) space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
#[serde(rename = "Point4d")]
#[serde(rename_all = "snake_case")]
pub struct Point4d<T = f32>
where
    kcep::Primitive: From<T>,
    T: kcep::Value,
{
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
    #[allow(missing_docs)]
    pub z: T,
    #[allow(missing_docs)]
    pub w: T,
}

impl From<euler::Vec3> for Point3d<f32> {
    fn from(v: euler::Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

/// A point in 2D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default, ExecutionPlanValue)]
#[serde(rename = "Point2d")]
#[serde(rename_all = "snake_case")]
pub struct Point2d<T = f32>
where
    kcep::Primitive: From<T>,
    T: kcep::Value,
{
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
}

/// An angle, with a specific unit.
#[derive(Clone, Copy, PartialEq, Debug, JsonSchema, Deserialize, Serialize)]
pub struct Angle {
    /// What unit is the measurement?
    pub unit: UnitAngle,
    /// The size of the angle, measured in the chosen unit.
    pub value: f64,
}

impl Angle {
    /// Converts a given angle to degrees.
    pub fn to_degrees(self) -> f64 {
        match self.unit {
            UnitAngle::Degrees => self.value,
            UnitAngle::Radians => self.value.to_degrees(),
        }
    }
    /// Create an angle in degrees.
    pub fn from_degrees(value: f64) -> Self {
        Self {
            unit: UnitAngle::Degrees,
            value,
        }
    }
    /// Create an angle in radians.
    pub fn from_radians(value: f64) -> Self {
        Self {
            unit: UnitAngle::Radians,
            value,
        }
    }
}

/// The type of scene selection change
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    ExecutionPlanValue,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum SceneSelectionType {
    /// Replaces the selection
    Replace,
    /// Adds to the selection
    Add,
    /// Removes from the selection
    Remove,
}

impl_string_enum_sql! {SceneSelectionType}

/// The type of scene's active tool
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "snake_case")]
pub enum SceneToolType {
    CameraRevolve,
    Select,
    Move,
    SketchLine,
    SketchTangentialArc,
    SketchCurve,
    SketchCurveMod,
}

impl_string_enum_sql! {SceneToolType}

/// The path component constraint bounds type
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    Default,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "snake_case")]
pub enum PathComponentConstraintBound {
    #[default]
    Unconstrained,
    PartiallyConstrained,
    FullyConstrained,
}

impl_string_enum_sql! {PathComponentConstraintBound}

/// The path component constraint type
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    Default,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "snake_case")]
pub enum PathComponentConstraintType {
    #[default]
    Unconstrained,
    Vertical,
    Horizontal,
    EqualLength,
    Parallel,
    AngleBetween,
}

impl_string_enum_sql! {PathComponentConstraintType}

/// The path component command type (within a Path)
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    ExecutionPlanValue,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "snake_case")]
pub enum PathCommand {
    MoveTo,
    LineTo,
    BezCurveTo,
    NurbsCurveTo,
    AddArc,
}

/// The type of entity
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    ExecutionPlanValue,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum EntityType {
    Entity,
    Object,
    Path,
    Curve,
    Solid2D,
    Solid3D,
    Edge,
    Face,
    Plane,
    Vertex,
}

/// The type of Curve (embedded within path)
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    Eq,
    PartialEq,
    Debug,
    JsonSchema,
    Deserialize,
    Serialize,
    Sequence,
    Clone,
    Ord,
    PartialOrd,
    ExecutionPlanValue,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Line,
    Arc,
    Nurbs,
}

/// A file to be exported to the client.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
pub struct ExportFile {
    /// The name of the file.
    pub name: String,
    /// The contents of the file, base64 encoded.
    pub contents: crate::base64::Base64Data,
}

/// The valid types of output file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
pub enum FileExportFormat {
    /// Autodesk Filmbox (FBX) format. <https://en.wikipedia.org/wiki/FBX>
    Fbx,
    /// Binary glTF 2.0.
    ///
    /// This is a single binary with .glb extension.
    ///
    /// This is better if you want a compressed format as opposed to the human readable
    /// glTF that lacks compression.
    Glb,
    /// glTF 2.0.
    /// Embedded glTF 2.0 (pretty printed).
    ///
    /// Single JSON file with .gltf extension binary data encoded as
    /// base64 data URIs.
    ///
    /// The JSON contents are pretty printed.
    ///
    /// It is human readable, single file, and you can view the
    /// diff easily in a git commit.
    Gltf,
    /// The OBJ file format. <https://en.wikipedia.org/wiki/Wavefront_.obj_file>
    /// It may or may not have an an attached material (mtl // mtllib) within the file,
    /// but we interact with it as if it does not.
    Obj,
    /// The PLY file format. <https://en.wikipedia.org/wiki/PLY_(file_format)>
    Ply,
    /// The STEP file format. <https://en.wikipedia.org/wiki/ISO_10303-21>
    Step,
    /// The STL file format. <https://en.wikipedia.org/wiki/STL_(file_format)>
    Stl,
}

impl_string_enum_sql! {FileExportFormat}

/// The valid types of source file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
pub enum FileImportFormat {
    /// Autodesk Filmbox (FBX) format. <https://en.wikipedia.org/wiki/FBX>
    Fbx,
    /// glTF 2.0.
    Gltf,
    /// The OBJ file format. <https://en.wikipedia.org/wiki/Wavefront_.obj_file>
    /// It may or may not have an an attached material (mtl // mtllib) within the file,
    /// but we interact with it as if it does not.
    Obj,
    /// The PLY file format. <https://en.wikipedia.org/wiki/PLY_(file_format)>
    Ply,
    /// SolidWorks part (SLDPRT) format.
    Sldprt,
    /// The STEP file format. <https://en.wikipedia.org/wiki/ISO_10303-21>
    Step,
    /// The STL file format. <https://en.wikipedia.org/wiki/STL_(file_format)>
    Stl,
}

/// The type of error sent by the KittyCAD graphics engine.
#[derive(Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum EngineErrorCode {
    /// User requested something geometrically or graphically impossible.
    /// Don't retry this request, as it's inherently impossible. Instead, read the error message
    /// and change your request.
    BadRequest = 1,
    /// Graphics engine failed to complete request, consider retrying
    InternalEngine,
}

impl From<EngineErrorCode> for http::StatusCode {
    fn from(e: EngineErrorCode) -> Self {
        match e {
            EngineErrorCode::BadRequest => Self::BAD_REQUEST,
            EngineErrorCode::InternalEngine => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl_string_enum_sql! {FileImportFormat}

/// An enum that contains the three global axes.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
#[serde(rename_all = "lowercase")]
pub enum GlobalAxis {
    /// The X axis
    X,
    /// The Y axis
    Y,
    /// The Z axis
    Z,
}
impl_string_enum_sql! {GlobalAxis}

// Enum: Connect Rust Enums to Cpp
// add our native c++ names for our cxx::ExternType implementation
#[cfg(feature = "cxx")]
impl_extern_type! {
    [Trivial]
    // File
    FileImportFormat = "Enums::_FileImportFormat"
    FileExportFormat = "Enums::_FileExportFormat"
    // Camera
    CameraDragInteractionType = "Enums::_CameraDragInteractionType"
    // Scene
    SceneSelectionType = "Enums::_SceneSelectionType"
    SceneToolType = "Enums::_SceneToolType"
    EntityType = "Enums::_EntityType"
    AnnotationType = "Enums::_AnnotationType"
    AnnotationTextAlignmentX = "Enums::_AnnotationTextAlignmentX"
    AnnotationTextAlignmentY = "Enums::_AnnotationTextAlignmentY"
    AnnotationLineEnd = "Enums::_AnnotationLineEnd"

    CurveType = "Enums::_CurveType"
    PathCommand = "Enums::_PathCommand"
    PathComponentConstraintBound = "Enums::_PathComponentConstraintBound"
    PathComponentConstraintType = "Enums::_PathComponentConstraintType"

    // Utils
    EngineErrorCode = "Enums::_ErrorCode"
    GlobalAxis = "Enums::_GlobalAxis"
}
