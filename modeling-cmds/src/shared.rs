use enum_iterator::Sequence;
use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cxx")]
use crate::impl_extern_type;
use crate::{length_unit::LengthUnit, units::UnitAngle};

/// What kind of cut to do
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum CutType {
    /// Round off an edge.
    #[default]
    Fillet,
    /// Cut away an edge.
    Chamfer,
}

/// A rotation defined by an axis, origin of rotation, and an angle.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Rotation {
    /// Rotation axis.
    /// Defaults to (0, 0, 1) (i.e. the Z axis).
    pub axis: Point3d<f64>,
    /// Rotate this far about the rotation axis.
    /// Defaults to zero (i.e. no rotation).
    pub angle: Angle,
    /// Origin of the rotation. If one isn't provided, the object will rotate about its own bounding box center.
    pub origin: OriginType,
}

impl Default for Rotation {
    /// z-axis, 0 degree angle, and local origin.
    fn default() -> Self {
        Self {
            axis: z_axis(),
            angle: Angle::default(),
            origin: OriginType::Local,
        }
    }
}

/// Ways to transform each solid being replicated in a repeating pattern.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct Transform {
    /// Translate the replica this far along each dimension.
    /// Defaults to zero vector (i.e. same position as the original).
    #[serde(default)]
    pub translate: Point3d<LengthUnit>,
    /// Scale the replica's size along each axis.
    /// Defaults to (1, 1, 1) (i.e. the same size as the original).
    #[serde(default = "same_scale")]
    pub scale: Point3d<f64>,
    /// Rotate the replica about the specified rotation axis and origin.
    /// Defaults to no rotation.
    #[serde(default)]
    pub rotation: Rotation,
    /// Whether to replicate the original solid in this instance.
    #[serde(default = "bool_true")]
    pub replicate: bool,
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

/// The type of origin
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum OriginType {
    /// Local Origin (center of object bounding box).
    #[default]
    Local,
    /// Global Origin (0, 0, 0).
    Global,
    /// Custom Origin (user specified point).
    Custom {
        /// Custom origin point.
        origin: Point3d<f64>,
    },
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
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationTextAlignmentX {
    Left,
    Center,
    Right,
}

/// Vertical Text alignment
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationTextAlignmentY {
    Bottom,
    Center,
    Top,
}

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

impl<T> Point3d<T> {
    /// Add the given `z` component to a 2D point to produce a 3D point.
    pub fn from_2d(Point2d { x, y }: Point2d<T>, z: T) -> Self {
        Self { x, y, z }
    }
    /// Takes some closure, and calls it on each component of this point.
    /// # Examples
    /// ```
    /// use kittycad_modeling_cmds::shared::Point3d;
    /// let p0 = Point3d{x: 1.0, y: 1.0, z: 1.0};
    /// assert_eq!(p0.map(|n| n*2.0), Point3d{x: 2.0, y: 2.0, z:2.0});
    /// ```
    pub fn map<U, F>(self, mut f: F) -> Point3d<U>
    where
        F: FnMut(T) -> U,
    {
        let Self { x, y, z } = self;
        Point3d {
            x: f(x),
            y: f(y),
            z: f(z),
        }
    }
}

/// Annotation line end type
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationLineEnd {
    None,
    Arrow,
}

/// The type of annotation
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationType {
    /// 2D annotation type (screen or planar space)
    T2D,
    /// 3D annotation type
    T3D,
}

/// The type of camera drag interaction.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PathSegment {
    /// A straight line segment.
    /// Goes from the current path "pen" to the given endpoint.
    Line {
        /// End point of the line.
        end: Point3d<LengthUnit>,
        ///Whether or not this line is a relative offset
        relative: bool,
    },
    /// A circular arc segment.
    /// Arcs can be drawn clockwise when start > end.
    Arc {
        /// Center of the circle
        center: Point2d<LengthUnit>,
        /// Radius of the circle
        radius: LengthUnit,
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
        control1: Point3d<LengthUnit>,
        /// Second control point.
        control2: Point3d<LengthUnit>,
        /// Final control point.
        end: Point3d<LengthUnit>,
        ///Whether or not this bezier is a relative offset
        relative: bool,
    },
    /// Adds a tangent arc from current pen position with the given radius and angle.
    TangentialArc {
        /// Radius of the arc.
        /// Not to be confused with Raiders of the Lost Ark.
        radius: LengthUnit,
        /// Offset of the arc. Negative values will arc clockwise.
        offset: Angle,
    },
    /// Adds a tangent arc from current pen position to the new position.
    /// Arcs will choose a clockwise or counter-clockwise direction based on the arc end position.
    TangentialArcTo {
        /// Where the arc should end.
        /// Must lie in the same plane as the current path pen position.
        /// Must not be colinear with current path pen position.
        to: Point3d<LengthUnit>,
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

impl<T> Point2d<T>
where
    T: Copy,
{
    /// Make a point where all components have the given value.
    pub fn uniform(value: T) -> Self {
        Self { x: value, y: value }
    }
}
impl<T> Point3d<T>
where
    T: Copy,
{
    /// Make a point where all components have the given value.
    pub fn uniform(value: T) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl<T> Point4d<T> {
    /// Takes some closure, and calls it on each component of this point.
    /// # Examples
    /// ```
    /// use kittycad_modeling_cmds::shared::Point4d;
    /// let p0 = Point4d{x: 1.0, y: 1.0, z: 1.0, w: 1.0};
    /// assert_eq!(p0.map(|n| n*2.0), Point4d{x: 2.0, y: 2.0, z: 2.0, w: 2.0});
    /// ```
    pub fn map<U, F>(self, mut f: F) -> Point4d<U>
    where
        F: FnMut(T) -> U,
    {
        let Self { x, y, z, w } = self;
        Point4d {
            x: f(x),
            y: f(y),
            z: f(z),
            w: f(w),
        }
    }
}
impl<T> Point4d<T>
where
    T: Copy,
{
    /// Make a point where all components have the given value.
    pub fn uniform(value: T) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }
    /// Make a point where the X, Y and Z components have the same value,
    /// but the W component has a different one.
    pub fn uniform_3d(xyz: T, w: T) -> Self {
        Self {
            x: xyz,
            y: xyz,
            z: xyz,
            w,
        }
    }
}

impl From<euler::Vec3> for Point3d<f32> {
    fn from(v: euler::Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl<T> From<[T; 3]> for Point3d<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

impl<T: PartialEq> PartialEq for Point4d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
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

impl<T: PartialEq> PartialEq for Point2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> From<[T; 2]> for Point2d<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

macro_rules! impl_arithmetic {
    ($typ:ident, $op:ident, $op_assign:ident, $method:ident, $method_assign:ident, $($i:ident),*) => {
        impl<T> std::ops::$op<$typ<T>> for $typ<T>
        where
            T: std::ops::$op<Output = T>,
        {
            type Output = $typ<T>;

            fn $method(self, rhs: $typ<T>) -> Self::Output {
                Self {
                    $(
                        $i: self.$i.$method(rhs.$i),
                    )*
                }
            }
        }
        impl<T> std::ops::$op_assign for $typ<T>
        where
            T: std::ops::$op_assign<T>,
        {

            fn $method_assign(&mut self, other: Self) {
                $(
                    self.$i.$method_assign(other.$i);
                )*
            }
        }
    };
}

impl_arithmetic!(Point2d, Add, AddAssign, add, add_assign, x, y);
impl_arithmetic!(Point3d, Add, AddAssign, add, add_assign, x, y, z);
impl_arithmetic!(Point2d, Sub, SubAssign, sub, sub_assign, x, y);
impl_arithmetic!(Point3d, Sub, SubAssign, sub, sub_assign, x, y, z);
impl_arithmetic!(Point2d, Mul, MulAssign, mul, mul_assign, x, y);
impl_arithmetic!(Point3d, Mul, MulAssign, mul, mul_assign, x, y, z);
impl_arithmetic!(Point2d, Div, DivAssign, div, div_assign, x, y);
impl_arithmetic!(Point3d, Div, DivAssign, div, div_assign, x, y, z);

impl<T> Point2d<T> {
    /// Add the given `z` component to a 2D point to produce a 3D point.
    pub fn with_z(self, z: T) -> Point3d<T> {
        let Self { x, y } = self;
        Point3d { x, y, z }
    }

    /// Takes some closure, and calls it on each component of this point.
    /// # Examples
    /// ```
    /// use kittycad_modeling_cmds::shared::Point2d;
    /// let p0 = Point2d{x: 1.0, y: 1.0};
    /// assert_eq!(p0.map(|n| n*2.0), Point2d{x: 2.0, y: 2.0});
    /// ```
    pub fn map<U, F>(self, mut f: F) -> Point2d<U>
    where
        F: FnMut(T) -> U,
    {
        let Self { x, y } = self;
        Point2d { x: f(x), y: f(y) }
    }
}

///A quaternion
pub type Quaternion = Point4d;

impl Default for Quaternion {
    /// (0, 0, 0, 1)
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
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
    /// Converts a given angle to radians.
    pub fn to_radians(self) -> f64 {
        match self.unit {
            UnitAngle::Degrees => self.value.to_radians(),
            UnitAngle::Radians => self.value,
        }
    }
    /// Create an angle in degrees.
    pub const fn from_degrees(value: f64) -> Self {
        Self {
            unit: UnitAngle::Degrees,
            value,
        }
    }
    /// Create an angle in radians.
    pub const fn from_radians(value: f64) -> Self {
        Self {
            unit: UnitAngle::Radians,
            value,
        }
    }
}

impl Angle {
    /// 360 degrees.
    pub const fn turn() -> Self {
        Self::from_degrees(360.0)
    }
    /// 180 degrees.
    pub const fn half_circle() -> Self {
        Self::from_degrees(180.0)
    }
    /// 90 degrees.
    pub const fn quarter_circle() -> Self {
        Self::from_degrees(90.0)
    }
    /// 0 degrees.
    pub const fn zero() -> Self {
        Self::from_degrees(0.0)
    }
}

/// 0 degrees.
impl Default for Angle {
    /// 0 degrees.
    fn default() -> Self {
        Self::from_degrees(0.0)
    }
}

impl std::ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            unit: UnitAngle::Degrees,
            value: self.to_degrees() + rhs.to_degrees(),
        }
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        match self.unit {
            UnitAngle::Degrees => {
                self.value += rhs.to_degrees();
            }
            UnitAngle::Radians => {
                self.value += rhs.to_radians();
            }
        }
    }
}

/// The type of scene selection change
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum SceneSelectionType {
    /// Replaces the selection
    Replace,
    /// Adds to the selection
    Add,
    /// Removes from the selection
    Remove,
}

/// The type of scene's active tool
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
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
#[serde(rename_all = "snake_case")]
pub enum PathComponentConstraintBound {
    #[default]
    Unconstrained,
    PartiallyConstrained,
    FullyConstrained,
}

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

/// The path component command type (within a Path)
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
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
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
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
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "snake_case")]
pub enum CurveType {
    Line,
    Arc,
    Nurbs,
}

/// A file to be exported to the client.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
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

/// The valid types of source file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
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

/// Camera settings including position, center, fov etc
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CameraSettings {
    ///Camera position (vantage)
    pub pos: Point3d,

    ///Camera's look-at center (center-pos gives viewing vector)
    pub center: Point3d,

    ///Camera's world-space up vector
    pub up: Point3d,

    ///The Camera's orientation (in the form of a quaternion)
    pub orientation: Quaternion,

    ///Camera's field-of-view angle (if ortho is false)
    pub fov_y: Option<f32>,

    ///The camera's ortho scale (derived from viewing distance if ortho is true)
    pub ortho_scale: Option<f32>,

    ///Whether or not the camera is in ortho mode
    pub ortho: bool,
}

impl From<CameraSettings> for crate::output::DefaultCameraZoom {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}
impl From<CameraSettings> for crate::output::CameraDragMove {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}
impl From<CameraSettings> for crate::output::CameraDragEnd {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}
impl From<CameraSettings> for crate::output::DefaultCameraGetSettings {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}
impl From<CameraSettings> for crate::output::ZoomToFit {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}
impl From<CameraSettings> for crate::output::ViewIsometric {
    fn from(settings: CameraSettings) -> Self {
        Self { settings }
    }
}

/// Defines a perspective view.
#[derive(Copy, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, PartialOrd, Default)]
#[serde(rename_all = "snake_case")]
pub struct PerspectiveCameraParameters {
    /// Camera frustum vertical field of view.
    pub fov_y: Option<f32>,
    /// Camera frustum near plane.
    pub z_near: Option<f32>,
    /// Camera frustum far plane.
    pub z_far: Option<f32>,
}

/// The global axes.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
pub enum GlobalAxis {
    /// The X axis
    X,
    /// The Y axis
    Y,
    /// The Z axis
    Z,
}

/// Possible types of faces which can be extruded from a 3D solid.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum ExtrusionFaceCapType {
    /// Uncapped.
    None,
    /// Capped on top.
    Top,
    /// Capped below.
    Bottom,
}

/// Post effect type
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
#[serde(rename_all = "lowercase")]
pub enum PostEffectType {
    Phosphor,
    Ssao,
    #[default]
    NoEffect,
}

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
    ExtrusionFaceCapType  = "Enums::_ExtrusionFaceCapType"

    // Utils
    EngineErrorCode = "Enums::_ErrorCode"
    GlobalAxis = "Enums::_GlobalAxis"
    OriginType = "Enums::_OriginType"

    // Graphics engine
    PostEffectType = "Enums::_PostEffectType"
}

fn bool_true() -> bool {
    true
}
fn same_scale() -> Point3d<f64> {
    let p = 1.0;
    Point3d { x: p, y: p, z: p }
}

fn z_axis() -> Point3d<f64> {
    Point3d { x: 0.0, y: 0.0, z: 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        let actual = Point2d { x: 1.0, y: 2.0 } + Point2d { x: 10.0, y: 20.0 };
        let expected = Point2d { x: 11.0, y: 22.0 };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_math_assign() {
        let mut p = Point2d { x: 1.0, y: 2.0 };
        p += Point2d { x: 10.0, y: 20.0 };
        let expected = Point2d { x: 11.0, y: 22.0 };
        assert_eq!(p, expected);
    }
}
