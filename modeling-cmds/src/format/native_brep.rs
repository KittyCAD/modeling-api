//! Mesh-free display geometry shared by Engine and browser tessellation.
//! World coordinates are meters, Z up; trim coordinates are surface parameters.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Current native B-rep wire protocol version.
pub const VERSION: u32 = 1;
/// Default native B-rep export filename.
pub const FILE_NAME: &str = "preview.brep.json";

fn deserialize_version<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let version = u32::deserialize(deserializer)?;
    if version != VERSION {
        return Err(serde::de::Error::custom(format!(
            "Unsupported native B-rep version: {version}"
        )));
    }
    Ok(version)
}

/// Coordinate system of a native B-rep scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[schemars(rename = "NativeBrepCoordinateSystem")]
pub enum CoordinateSystem {
    /// Meters with positive Z up.
    ZUp,
}

/// A complete replacement for the preceding scene preview.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepSceneSnapshot")]
pub struct SceneSnapshot {
    #[serde(deserialize_with = "deserialize_version")]
    /// Wire protocol version; currently 1.
    #[schemars(range(min = 1, max = 1))]
    pub version: u32,
    /// World coordinate system.
    pub coordinate_system: CoordinateSystem,
    /// Bodies in this scene.
    pub bodies: Vec<Body>,
    /// Bodies omitted by the producer and their diagnostic messages.
    pub errors: Vec<String>,
}

/// One identified body in a scene preview.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "NativeBrepBody")]
pub struct Body {
    /// Stable engine body identifier.
    pub id: String,
    /// Unmeshed body geometry.
    pub brep: Brep,
}

/// A world-space point or direction, in meters.
pub type Point = [f64; 3];
/// A point or direction in surface parameter coordinates.
pub type Uv = [f64; 2];
/// An entity index paired with its orientation (1 or -1).
pub type Oriented = (usize, i8);

/// A single-body native B-rep snapshot.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepSnapshot")]
pub struct Snapshot {
    #[serde(deserialize_with = "deserialize_version")]
    /// Wire protocol version; currently 1.
    #[schemars(range(min = 1, max = 1))]
    pub version: u32,
    /// Unmeshed body geometry.
    pub brep: Brep,
}

/// Unmeshed surfaces and their trimmed face topology.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepBrep")]
pub struct Brep {
    /// Surface definitions referenced by faces.
    pub surfaces: Vec<Surface>,
    /// Oriented, trimmed faces.
    pub faces: Vec<Face>,
    /// Oriented trim loops referenced by this face, or loop definitions in a body.
    pub loops: Vec<TrimLoop>,
    #[serde(rename = "curves2D")]
    /// Trim curve definitions.
    pub curves2d: Vec<Curve>,
}

/// An analytic plane or rational spline surface.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[schemars(rename = "NativeBrepSurface")]
pub enum Surface {
    /// An affine plane.
    Plane {
        /// Geometry parameters.
        plane: Plane,
    },
    /// A rational spline.
    Nurbs {
        /// Geometry parameters.
        nurbs: Nurbs,
    },
}

/// Affine plane basis in world coordinates.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepPlane")]
pub struct Plane {
    #[serde(default)]
    /// Origin of the local basis.
    pub origin: Point,
    #[serde(default = "x_axis")]
    /// Local X direction.
    pub x_axis: Point,
    #[serde(default = "y_axis")]
    /// Local Y direction.
    pub y_axis: Point,
}
fn x_axis() -> Point {
    [1.0, 0.0, 0.0]
}
fn y_axis() -> Point {
    [0.0, 1.0, 0.0]
}

/// Rational tensor-product spline surface.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepNurbs")]
pub struct Nurbs {
    #[serde(default)]
    /// Whether the surface closes in each parameter direction.
    pub closed: [bool; 2],
    /// Control points in the corresponding coordinate space.
    pub control_points: Vec<Point>,
    #[serde(default)]
    /// Homogeneous control weights; empty means all weights are one.
    pub weights: Vec<f64>,
    /// Spline knots, concatenated by direction for surfaces.
    pub knot_vector: Vec<f64>,
    /// Control point counts in each surface parameter direction.
    pub num_control_points: [usize; 2],
    /// Knot counts in each surface parameter direction.
    pub num_knots: [usize; 2],
    /// Spline order (degree plus one).
    pub order: [usize; 2],
}

/// An oriented surface bounded by trim loops.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "NativeBrepFace")]
pub struct Face {
    /// Surface index and orientation.
    #[schemars(with = "[i64; 2]")]
    pub surface: Oriented,
    /// Oriented trim loops referenced by this face, or loop definitions in a body.
    #[schemars(with = "Vec<[i64; 2]>")]
    pub loops: Vec<Oriented>,
}
/// A face boundary in surface parameter coordinates.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "NativeBrepTrimLoop")]
pub struct TrimLoop {
    #[serde(default)]
    /// Ordered trim traces bounding this loop.
    pub traces: Vec<Trace>,
}
/// An oriented trim curve and its parameter interval.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "NativeBrepTrace")]
pub struct Trace {
    /// Curve index and orientation.
    #[schemars(with = "[i64; 2]")]
    pub curve: Oriented,
    /// Start and end parameters along the trim curve.
    pub t: [f64; 2],
}
/// A trim curve in surface parameter coordinates.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[schemars(rename = "NativeBrepCurve")]
pub enum Curve {
    /// An infinite line restricted by its trace interval.
    Line {
        /// Geometry parameters.
        line: Line,
    },
    /// A circle restricted by its trace interval.
    Circle {
        /// Geometry parameters.
        circle: Circle,
    },
    /// A rational spline.
    Nurbs {
        /// Geometry parameters.
        nurbs: NurbsCurve,
    },
}
/// A line in surface parameter coordinates.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "NativeBrepLine")]
pub struct Line {
    #[serde(default)]
    /// Origin of the local basis.
    pub origin: Uv,
    /// Direction of the line.
    pub direction: Uv,
}
/// A circle in surface parameter coordinates.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepCircle")]
pub struct Circle {
    #[serde(default)]
    /// Origin of the local basis.
    pub origin: Uv,
    /// Radius in surface parameter coordinates.
    pub radius: f64,
    #[serde(default = "uv_x_axis")]
    /// Local X direction.
    pub x_axis: Uv,
    #[serde(default = "uv_y_axis")]
    /// Local Y direction.
    pub y_axis: Uv,
}
fn uv_x_axis() -> Uv {
    [1.0, 0.0]
}
fn uv_y_axis() -> Uv {
    [0.0, 1.0]
}
/// A rational spline trim curve.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "NativeBrepNurbsCurve")]
pub struct NurbsCurve {
    /// Control points in the corresponding coordinate space.
    pub control_points: Vec<Uv>,
    #[serde(default)]
    /// Homogeneous control weights; empty means all weights are one.
    pub weights: Vec<f64>,
    /// Spline knots, concatenated by direction for surfaces.
    pub knot_vector: Vec<f64>,
    /// Spline order (degree plus one).
    pub order: usize,
}

/// Encode already-serialized bodies without parsing or rounding their geometry.
///
/// Each body must be valid JSON conforming to [`Brep`], produced by the trusted
/// native encoder. This function escapes envelope strings but deliberately does
/// not validate raw geometry; compatibility fixtures check the native producer.
pub fn encode_scene(bodies: &[(&str, &[u8])], errors: &[String]) -> serde_json::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(
        bodies
            .iter()
            .map(|(id, body)| id.len() + body.len() + 24)
            .sum::<usize>()
            + 128,
    );
    out.extend_from_slice(b"{\"version\":");
    serde_json::to_writer(&mut out, &VERSION)?;
    out.extend_from_slice(b",\"coordinateSystem\":");
    serde_json::to_writer(&mut out, &CoordinateSystem::ZUp)?;
    out.extend_from_slice(b",\"bodies\":[");
    for (i, (id, body)) in bodies.iter().enumerate() {
        if i != 0 {
            out.push(b',');
        }
        out.extend_from_slice(b"{\"id\":");
        serde_json::to_writer(&mut out, id)?;
        out.extend_from_slice(b",\"brep\":");
        out.extend_from_slice(body);
        out.push(b'}');
    }
    out.extend_from_slice(b"],\"errors\":");
    serde_json::to_writer(&mut out, errors)?;
    out.push(b'}');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_capture_is_compatible_and_keeps_exact_geometry_bytes() {
        // Captured from the C++ producer, not synthesized from these Rust types.
        let body = include_bytes!("../../tests/fixtures/native-brep/sphere.brep.json");
        let id = "body\"\\\n";
        let errors = vec!["skipped body\n\"detail\"".to_owned()];
        let packet = encode_scene(&[(id, body)], &errors).unwrap();
        let scene: SceneSnapshot = serde_json::from_slice(&packet).unwrap();
        assert_eq!(scene.bodies[0].id, id);
        assert!(!scene.bodies[0].brep.faces.is_empty());
        assert_eq!(scene.errors, errors);
        assert!(packet.windows(body.len()).any(|part| part == body));
        let precise =
            br#"{"surfaces":[],"faces":[],"loops":[],"curves2D":[],"precise":1.2345678901234567,"tiny":1e-200}"#;
        let multiple = encode_scene(&[(id, body), ("precise", precise)], &[]).unwrap();
        let decoded: SceneSnapshot = serde_json::from_slice(&multiple).unwrap();
        assert_eq!(decoded.bodies.len(), 2);
        assert!(multiple.windows(precise.len()).any(|part| part == precise));
        let empty: SceneSnapshot = serde_json::from_slice(&encode_scene(&[], &[]).unwrap()).unwrap();
        assert!(empty.bodies.is_empty());
    }

    #[test]
    fn unsupported_protocol_versions_and_coordinate_systems_are_rejected() {
        let packet = encode_scene(&[], &[]).unwrap();
        let mut scene: serde_json::Value = serde_json::from_slice(&packet).unwrap();
        scene["version"] = (VERSION + 1).into();
        assert!(serde_json::from_value::<SceneSnapshot>(scene.clone()).is_err());
        scene["version"] = VERSION.into();
        scene["coordinateSystem"] = "y-up".into();
        assert!(serde_json::from_value::<SceneSnapshot>(scene).is_err());
    }
    #[test]
    fn native_geometry_schemas_are_openapi_compatible() {
        let mut generator = schemars::gen::SchemaSettings::openapi3().into_generator();
        generator.subschema_for::<SceneSnapshot>();
        generator.subschema_for::<Snapshot>();
        let json = serde_json::to_value(generator.take_definitions()).unwrap();
        assert!(json.get("NativeBrepSceneSnapshot").is_some());
        assert!(json.get("NativeBrepSnapshot").is_some());
        assert_eq!(json["NativeBrepSceneSnapshot"]["properties"]["version"]["maximum"], 1.0);
        assert_eq!(
            json["NativeBrepFace"]["properties"]["surface"]["items"]["type"],
            "integer"
        );
    }
}
