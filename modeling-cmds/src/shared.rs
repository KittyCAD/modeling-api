use std::str::FromStr;

use diesel::{mysql::Mysql, serialize::ToSql, sql_types::Text};
use diesel_derives::{AsExpression, FromSqlRow};
use enum_iterator::Sequence;
use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::units::UnitAngle;

// A helper macro for allowing enums of only strings to be saved to the database.
macro_rules! impl_string_enum_sql {
    {$name:ident} => {
        impl diesel::serialize::ToSql<Text, Mysql> for $name {
            fn to_sql<'a>(&'a self, out: &mut diesel::serialize::Output<'a, '_, Mysql>) -> diesel::serialize::Result {
                <String as ToSql<Text, Mysql>>::to_sql(&self.to_string(), &mut out.reborrow())
            }
        }

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
    pub position: Option<Point3d>,
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

/// An RGBA color
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationTextAlignmentY {
    Bottom,
    Center,
    Top,
}

impl_string_enum_sql! {AnnotationTextAlignmentY}

/// A point in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename = "Point3d")]
#[serde(rename_all = "snake_case")]
pub struct Point3d<T = f32> {
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
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationLineEnd {
    None,
    Arrow,
}

impl_string_enum_sql! {AnnotationLineEnd}

/// The type of annotation
#[derive(
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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
        /// Start of the arc along circle's perimeter, in degrees.
        /// Deprecated: use `start` instead.
        #[deprecated(note = "use `start` instead, because it allows either degrees or radians")]
        #[serde(rename = "angle_start")]
        degrees_start: f64,
        /// End of the arc along circle's perimeter, in degrees.
        /// Deprecated: use `end` instead.
        #[deprecated(note = "use `end` instead, because it allows either degrees or radians")]
        #[serde(rename = "angle_end")]
        degrees_end: f64,
        /// Start of the arc along circle's perimeter.
        /// If not given, this will use `degrees_start` instead.
        #[serde(default)]
        start: Option<Angle>,
        /// End of the arc along circle's perimeter.
        /// If not given, this will use `degrees_end` instead.
        #[serde(default)]
        end: Option<Angle>,
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "Point4d")]
#[serde(rename_all = "snake_case")]
pub struct Point4d<T = f32> {
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
    #[allow(missing_docs)]
    pub z: T,
    #[allow(missing_docs)]
    pub w: T,
}

impl From<euler::Vec3> for Point3d {
    fn from(v: euler::Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

/// A point in 2D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename = "Point2d")]
#[serde(rename_all = "snake_case")]
pub struct Point2d<T = f32> {
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
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
    AsExpression,
    FromSqlRow,
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
#[diesel(sql_type = Text)]
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
    AsExpression,
    FromSqlRow,
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
#[diesel(sql_type = Text)]
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

/// Data item selection.
#[derive(Clone, Debug, Default, Display, Eq, FromStr, Hash, PartialEq, JsonSchema, Deserialize, Serialize)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Selection {
    /// Visit the default scene.
    #[default]
    DefaultScene,

    /// Visit the indexed scene.
    #[display("{}: {index}")]
    SceneByIndex {
        /// The index.
        index: usize,
    },

    /// Visit the first scene with the given name.
    #[display("{}: {name}")]
    SceneByName {
        /// The name.
        name: String,
    },

    /// Visit the indexed mesh.
    #[display("{}: {index}")]
    MeshByIndex {
        /// The index.
        index: usize,
    },

    /// Visit the first mesh with the given name.
    #[display("{}: {name}")]
    MeshByName {
        /// The name.
        name: String,
    },
}

/// The path component command type (within a Path)
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
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
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "lowercase")]
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
}

/// The type of Curve (embedded within path)
#[allow(missing_docs)]
#[derive(
    Display,
    FromStr,
    Copy,
    AsExpression,
    FromSqlRow,
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
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Line,
    Arc,
    Nurbs,
}

/// A file to be exported to the client.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportFile {
    /// The name of the file.
    pub name: String,
    /// The contents of the file, base64 encoded.
    pub contents: crate::base64::Base64Data,
}
