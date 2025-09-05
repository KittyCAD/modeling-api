use enum_iterator::Sequence;
use parse_display_derive::{Display, FromStr};
pub use point::{Point2d, Point3d, Point4d, Quaternion};
use schemars::{schema::SchemaObject, JsonSchema};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "cxx")]
use crate::impl_extern_type;
use crate::{length_unit::LengthUnit, output::ExtrusionFaceInfo, units::UnitAngle};

mod point;

/// What kind of cut to perform when cutting an edge.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum CutType {
    /// Round off an edge.
    #[default]
    Fillet,
    /// Cut away an edge.
    Chamfer {
        /// The second length affects the edge length of the second face of the cut.
        second_length: Option<LengthUnit>,
        /// The angle of the chamfer, default is 45deg.
        angle: Option<Angle>,
        /// If true, the second length or angle is applied to the other face of the cut.
        swap: bool,
    },
    /// A custom cut profile.
    Custom {
        /// The path that will be used for the custom profile.
        path: Uuid,
    },
}

/// A rotation defined by an axis, origin of rotation, and an angle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: same_scale(),
            replicate: true,
            translate: Default::default(),
            rotation: Rotation::default(),
        }
    }
}

/// Options for annotations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct AnnotationLineEndOptions {
    /// How to style the start of the annotation line.
    pub start: AnnotationLineEnd,
    /// How to style the end of the annotation line.
    pub end: AnnotationLineEnd,
}

/// Options for annotation text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum AnnotationTextAlignmentY {
    Bottom,
    Center,
    Top,
}

/// Annotation line end type
#[allow(missing_docs)]
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum AnnotationLineEnd {
    None,
    Arrow,
}

/// The type of annotation
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum CameraDragInteractionType {
    /// Camera pan
    Pan,
    /// Camera rotate (spherical camera revolve/orbit)
    Rotate,
    /// Camera rotate (trackball with 3 degrees of freedom)
    RotateTrackball,
    /// Camera zoom (increase or decrease distance to reference point center)
    Zoom,
}

/// A segment of a path.
/// Paths are composed of many segments.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
    ///Adds an arc from the current position that goes through the given interior point and ends at the given end position
    ArcTo {
        /// Interior point of the arc.
        interior: Point3d<LengthUnit>,
        /// End point of the arc.
        end: Point3d<LengthUnit>,
        ///Whether or not interior and end are relative to the previous path position
        relative: bool,
    },
    ///Adds a circular involute from the current position that goes through the given end_radius
    ///and is rotated around the current point by angle.
    CircularInvolute {
        ///The involute is described between two circles, start_radius is the radius of the inner
        ///circle.
        start_radius: LengthUnit,
        ///The involute is described between two circles, end_radius is the radius of the outer
        ///circle.
        end_radius: LengthUnit,
        ///The angle to rotate the involute by. A value of zero will produce a curve with a tangent
        ///along the x-axis at the start point of the curve.
        angle: Angle,
        ///If reverse is true, the segment will start
        ///from the end of the involute, otherwise it will start from that start.
        reverse: bool,
    },
    ///Adds an elliptical arc segment.
    Ellipse {
        /// The center point of the ellipse.
        center: Point2d<LengthUnit>,
        /// Major axis of the ellipse.
        major_axis: Point2d<LengthUnit>,
        /// Minor radius of the ellipse.
        minor_radius: LengthUnit,
        /// Start of the path along the perimeter of the ellipse.
        start_angle: Angle,
        /// End of the path along the perimeter of the ellipse.
        end_angle: Angle,
    },
    ///Adds a generic conic section specified by the end point, interior point and tangents at the
    ///start and end of the section.
    ConicTo {
        /// Interior point that lies on the conic.
        interior: Point2d<LengthUnit>,
        /// End point of the conic.
        end: Point2d<LengthUnit>,
        /// Tangent at the start of the conic.
        start_tangent: Point2d<LengthUnit>,
        /// Tangent at the end of the conic.
        end_tangent: Point2d<LengthUnit>,
        /// Whether or not the interior and end points are relative to the previous path position.
        relative: bool,
    },
}

/// An angle, with a specific unit.
#[derive(Clone, Copy, PartialEq, Debug, JsonSchema, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
        Self::zero()
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.unit, other.unit) {
            // Avoid unnecessary floating point operations.
            (UnitAngle::Degrees, UnitAngle::Degrees) => self.value.partial_cmp(&other.value),
            (UnitAngle::Radians, UnitAngle::Radians) => self.value.partial_cmp(&other.value),
            _ => self.to_degrees().partial_cmp(&other.to_degrees()),
        }
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum CurveType {
    Line,
    Arc,
    Nurbs,
}

/// A file to be exported to the client.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass)]
pub struct ExportFile {
    /// The name of the file.
    pub name: String,
    /// The contents of the file, base64 encoded.
    pub contents: crate::base64::Base64Data,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pyo3::pymethods]
impl ExportFile {
    #[getter]
    fn contents(&self) -> Vec<u8> {
        self.contents.0.clone()
    }

    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// The valid types of output file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
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

/// The valid types of 2D output file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum FileExportFormat2d {
    /// AutoCAD drawing interchange format.
    Dxf,
}

/// The valid types of source file formats.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd, Sequence,
)]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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

/// Extrusion method determining if the extrusion will be part of the existing object or an
/// entirely new object.
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum ExtrudeMethod {
    /// Create a new object that is not connected to the object it is extruded from. This will
    /// result in two objects after the operation.
    New,
    /// This extrusion will be part of object it is extruded from. This will result in one object
    /// after the operation.
    #[default]
    Merge,
}

/// IDs for the extruded faces.
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct ExtrudedFaceInfo {
    /// The face made from the original 2D shape being extruded.
    /// If the solid is extruded from a shape which already has an ID
    /// (e.g. extruding something which was sketched on a face), this
    /// doesn't need to be sent.
    pub bottom: Option<Uuid>,
    /// Top face of the extrusion (parallel and further away from the original 2D shape being extruded).
    pub top: Uuid,
    /// Any intermediate sides between the top and bottom.
    pub sides: Vec<SideFace>,
}

/// IDs for a side face, extruded from the path of some sketch/2D shape.
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct SideFace {
    /// ID of the path this face is being extruded from.
    pub path_id: Uuid,
    /// Desired ID for the resulting face.
    pub face_id: Uuid,
}

/// Camera settings including position, center, fov etc
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum WorldCoordinateSystem {
    #[default]
    RightHandedUpZ,
    RightHandedUpY,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct CameraViewState {
    pub pivot_rotation: Quaternion,
    pub pivot_position: Point3d,
    pub eye_offset: f32,
    pub fov_y: f32,
    pub ortho_scale_factor: f32,
    pub is_ortho: bool,
    pub ortho_scale_enabled: bool,
    pub world_coord_system: WorldCoordinateSystem,
}

impl Default for CameraViewState {
    fn default() -> Self {
        CameraViewState {
            pivot_rotation: Default::default(),
            pivot_position: Default::default(),
            eye_offset: 10.0,
            fov_y: 45.0,
            ortho_scale_factor: 1.6,
            is_ortho: false,
            ortho_scale_enabled: true,
            world_coord_system: Default::default(),
        }
    }
}

#[cfg(feature = "cxx")]
impl_extern_type! {
    [Trivial]
    CameraViewState = "Endpoints::CameraViewState"
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
impl From<CameraSettings> for crate::output::OrientToFace {
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct PerspectiveCameraParameters {
    /// Camera frustum vertical field of view.
    pub fov_y: Option<f32>,
    /// Camera frustum near plane.
    pub z_near: Option<f32>,
    /// Camera frustum far plane.
    pub z_far: Option<f32>,
}

/// A type of camera movement applied after certain camera operations
#[derive(
    Default,
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
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum CameraMovement {
    /// Adjusts the camera position during the camera operation
    #[default]
    Vantage,
    /// Keeps the camera position in place
    None,
}

/// The global axes.
#[derive(
    Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Sequence, Clone, Ord, PartialOrd,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[repr(u8)]
pub enum ExtrusionFaceCapType {
    /// Uncapped.
    None,
    /// Capped on top.
    Top,
    /// Capped below.
    Bottom,
    /// Capped on both ends.
    Both,
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
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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
    Point3d::uniform(1.0)
}

fn z_axis() -> Point3d<f64> {
    Point3d { x: 0.0, y: 0.0, z: 1.0 }
}

impl ExtrudedFaceInfo {
    /// Converts from the representation used in the Extrude modeling command,
    /// to a flat representation.
    pub fn list_faces(self) -> Vec<ExtrusionFaceInfo> {
        let mut face_infos: Vec<_> = self
            .sides
            .into_iter()
            .map(|side| ExtrusionFaceInfo {
                curve_id: Some(side.path_id),
                face_id: Some(side.face_id),
                cap: ExtrusionFaceCapType::None,
            })
            .collect();
        face_infos.push(ExtrusionFaceInfo {
            curve_id: None,
            face_id: Some(self.top),
            cap: ExtrusionFaceCapType::Top,
        });
        if let Some(bottom) = self.bottom {
            face_infos.push(ExtrusionFaceInfo {
                curve_id: None,
                face_id: Some(bottom),
                cap: ExtrusionFaceCapType::Bottom,
            });
        }
        face_infos
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::*;

    #[test]
    fn check_transformby_deprecated() {
        let s = schema_for!(TransformBy<Point3d>);
        let pretty = serde_json::to_string_pretty(&s).unwrap();
        println!("{pretty}");
        let tests: Vec<(OriginType, TransformBy<Point3d>)> = vec![
            // get_origin should fall back to `is_local`, because `origin` is none.
            (
                OriginType::Local,
                TransformBy {
                    property: Point3d::default(),
                    set: true,
                    #[allow(deprecated)] // still need to test deprecated code
                    is_local: true,
                    origin: None,
                },
            ),
            // get_origin should ignore `is_local`, because `origin` is given.
            // test the case where origin is not custom
            (
                OriginType::Local,
                TransformBy {
                    property: Point3d::default(),
                    set: true,
                    #[allow(deprecated)] // still need to test deprecated code
                    is_local: false,
                    origin: Some(OriginType::Local),
                },
            ),
            // get_origin should ignore `is_local`, because `origin` is given.
            // test the case where origin is custom.
            (
                OriginType::Custom {
                    origin: Point3d::uniform(2.0),
                },
                TransformBy {
                    property: Point3d::default(),
                    set: true,
                    #[allow(deprecated)] // still need to test deprecated code
                    is_local: false,
                    origin: Some(OriginType::Custom{origin: Point3d::uniform(2.0)}),
                },
            ),
        ];
        for (expected, input) in tests {
            let actual = input.get_origin();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_angle_comparison() {
        let a = Angle::from_degrees(90.0);
        assert!(a < Angle::from_degrees(91.0));
        assert!(a > Angle::from_degrees(89.0));
        assert!(a <= Angle::from_degrees(90.0));
        assert!(a >= Angle::from_degrees(90.0));
        let b = Angle::from_radians(std::f64::consts::FRAC_PI_4);
        assert!(b < Angle::from_radians(std::f64::consts::FRAC_PI_2));
        assert!(b > Angle::from_radians(std::f64::consts::FRAC_PI_8));
        assert!(b <= Angle::from_radians(std::f64::consts::FRAC_PI_4));
        assert!(b >= Angle::from_radians(std::f64::consts::FRAC_PI_4));
        // Mixed units.
        assert!(a > b);
        assert!(a >= b);
        assert!(b < a);
        assert!(b <= a);
        let c = Angle::from_radians(std::f64::consts::FRAC_PI_2 * 3.0);
        assert!(a < c);
        assert!(a <= c);
        assert!(c > a);
        assert!(c >= a);
    }
}

/// How a property of an object should be transformed.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
#[schemars(rename = "TransformByFor{T}")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct TransformBy<T> {
    /// The scale, or rotation, or translation.
    pub property: T,
    /// If true, overwrite the previous value with this.
    /// If false, the previous value will be modified.
    /// E.g. when translating, `set=true` will set a new location,
    /// and `set=false` will translate the current location by the given X/Y/Z.
    pub set: bool,
    /// If true, the transform is applied in local space.
    /// If false, the transform is applied in global space.
    #[deprecated(note = "Use the `origin` field instead.")]
    pub is_local: bool,
    /// What to use as the origin for the transformation.
    /// If not provided, will fall back to local or global origin, depending on
    /// whatever the `is_local` field was set to.
    #[serde(default)]
    pub origin: Option<OriginType>,
}

impl<T> TransformBy<T> {
    /// Get the origin of this transformation.
    /// Reads from the `origin` field if it's set, otherwise
    /// falls back to the `is_local` field.
    pub fn get_origin(&self) -> OriginType {
        if let Some(origin) = self.origin {
            return origin;
        }
        #[expect(
            deprecated,
            reason = "Must fall back to the deprecated field if the API client isn't using the new field yet."
        )]
        if self.is_local {
            OriginType::Local
        } else {
            OriginType::Global
        }
    }
}

/// Container that holds a translate, rotate and scale.
/// Defaults to no change, everything stays the same (i.e. the identity function).
#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, Serialize, Default)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub struct ComponentTransform {
    /// Translate component of the transform.
    pub translate: Option<TransformBy<Point3d<LengthUnit>>>,
    /// Rotate component of the transform.
    /// The rotation is specified as a roll, pitch, yaw.
    pub rotate_rpy: Option<TransformBy<Point3d<f64>>>,
    /// Rotate component of the transform.
    /// The rotation is specified as an axis and an angle (xyz are the components of the axis, w is
    /// the angle in degrees).
    pub rotate_angle_axis: Option<TransformBy<Point4d<f64>>>,
    /// Scale component of the transform.
    pub scale: Option<TransformBy<Point3d<f64>>>,
}

///If bidirectional or symmetric operations are needed this enum encapsulates the required
///information.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum Opposite<T> {
    /// No opposite. The operation will only occur on one side.
    #[default]
    None,
    /// Operation will occur from both sides, with the same value.
    Symmetric,
    /// Operation will occur from both sides, with this value for the opposite.
    Other(T),
}

impl<T: JsonSchema> JsonSchema for Opposite<T> {
    fn schema_name() -> String {
        format!("OppositeFor{}", T::schema_name())
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}::Opposite<{}>", module_path!(), T::schema_id()))
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

/// What strategy (algorithm) should be used for cutting?
/// Defaults to Automatic.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum CutStrategy {
    /// Basic fillet cut. This has limitations, like the filletted edges
    /// can't touch each other. But it's very fast and simple.
    Basic,
    /// More complicated fillet cut. It works for more use-cases, like
    /// edges that touch each other. But it's slower than the Basic method.
    Csg,
    /// Tries the Basic method, and if that doesn't work, tries the CSG strategy.
    #[default]
    Automatic,
}

/// What is the given geometry relative to?
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum RelativeTo {
    /// Local/relative to a position centered within the plane being sketched on
    #[default]
    SketchPlane,
    /// Local/relative to the trajectory curve
    TrajectoryCurve,
}
