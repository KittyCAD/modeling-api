use serde_json::{json, Value};

use crate::{ok_response::OkModelingCmdResponse, output, shared::DrawingProjectionCurveSource, ModelingCmd};

fn command() -> Value {
    json!({
        "type": "compute_drawing_projection",
        "object_id": "00000000-0000-0000-0000-000000000001",
        "frame": {
            "origin": {"x": 25.4, "y": 0.0, "z": 0.0},
            "x_axis": {"x": 1.0, "y": 0.0, "z": 0.0},
            "y_axis": {"x": 0.0, "y": 1.0, "z": 0.0}
        }
    })
}

#[test]
fn drawing_projection_minimal_command_omits_overrides() {
    let wire = command();
    let parsed: ModelingCmd = serde_json::from_value(wire.clone()).unwrap();
    let ModelingCmd::ComputeDrawingProjection(ref projection) = parsed else {
        panic!("wrong command variant");
    };
    assert!(projection.hidden_lines.is_none());
    assert!(projection.resolution.is_none());
    assert!(projection.tolerance.is_none());
    assert!(
        !parsed.is_safe_to_batch(),
        "a projection query must return its own result"
    );
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
}

#[test]
fn drawing_projection_explicit_options_round_trip() {
    for hidden_lines in ["visible_only", "include_hidden"] {
        let mut wire = command();
        wire["hidden_lines"] = json!(hidden_lines);
        wire["resolution"] = json!(19);
        wire["tolerance"] = json!(0.00001);
        let parsed: ModelingCmd = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    }
}

#[test]
fn drawing_projection_geometry_and_source_variants_round_trip() {
    let wire = json!({
        "type": "compute_drawing_projection",
        "data": {"curves": [
            {
                "geometry": {"type": "line", "start": {"x": 0.0, "y": 0.0}, "end": {"x": 10.0, "y": 0.0}},
                "visibility": "visible",
                "source": {"type": "edge", "edge_id": "00000000-0000-0000-0000-000000000002", "edge_index": 3, "section_index": 0},
                "segment_index": 0
            },
            {
                "geometry": {"type": "circular_arc", "origin": {"x": 0.0, "y": 0.0}, "radius": 5.0,
                    "start_angle": 0.0, "end_angle": std::f64::consts::TAU, "counter_clockwise": true},
                "visibility": "hidden",
                "source": {"type": "silhouette", "face_index": 2, "silhouette_index": 1},
                "segment_index": 0
            },
            {
                "geometry": {"type": "nurbs", "control_points": [{"x": 0.0, "y": 0.0}, {"x": 1.0, "y": 1.0}, {"x": 2.0, "y": 0.0}],
                    "weights": [1.0, 0.5, 1.0], "knot_vector": [0.0, 0.0, 0.0, 1.0, 1.0, 1.0], "order": 3},
                "visibility": "visible",
                "source": {"type": "silhouette", "face_id": "00000000-0000-0000-0000-000000000003", "face_index": 2, "silhouette_index": 1},
                "segment_index": 1
            }
        ]}
    });
    let parsed: OkModelingCmdResponse = serde_json::from_value(wire.clone()).unwrap();
    assert!(matches!(&parsed, OkModelingCmdResponse::ComputeDrawingProjection(_)));
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
}

#[test]
fn drawing_projection_non_rational_nurbs_omits_weights() {
    let wire = json!({"type": "nurbs", "control_points": [{"x": 0.0, "y": 0.0}, {"x": 1.0, "y": 0.0}],
        "knot_vector": [0.0, 0.0, 1.0, 1.0], "order": 2});
    let parsed: crate::shared::DrawingProjectionCurveGeometry = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
}

#[test]
fn drawing_projection_source_requires_topology_and_component_identity() {
    for source in [
        json!({"type": "edge", "section_index": 0}),
        json!({"type": "edge", "edge_index": 2}),
        json!({"type": "silhouette", "face_index": 2}),
        json!({"type": "silhouette", "edge_index": 2, "silhouette_index": 0}),
    ] {
        assert!(serde_json::from_value::<DrawingProjectionCurveSource>(source).is_err());
    }
}

#[test]
fn drawing_projection_empty_result_is_valid() {
    let wire = json!({"curves": []});
    let parsed: output::ComputeDrawingProjection = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
}

#[cfg(feature = "ts-rs")]
#[test]
fn drawing_projection_typescript_preserves_source_discrimination() {
    use ts_rs::TS;

    let config = ts_rs::Config::default();
    let source = DrawingProjectionCurveSource::decl(&config);
    assert!(source.contains("\"type\": \"edge\""));
    assert!(source.contains("\"type\": \"silhouette\""));
    assert!(source.contains("section_index: number"));
    assert!(source.contains("silhouette_index: number"));
    assert!(ModelingCmd::decl(&config).contains("compute_drawing_projection"));
}
