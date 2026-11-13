//! Compatibility tests for optional supplemental annotation text.

use kittycad_modeling_cmds::shared::{AnnotationFeatureControl, AnnotationFeatureTag};
use serde_json::{json, Value};

fn legacy_annotation() -> Value {
    json!({
        "leader_type": "arrow",
        "plane_id": "00000000-0000-0000-0000-000000000000",
        "offset": { "x": 10.0, "y": 20.0 },
        "precision": 3,
        "font_scale": 1.0,
        "font_point_size": 36,
        "key": "",
        "value": "Note",
        "show_key": false
    })
}

#[test]
fn supplemental_text_is_optional_for_existing_annotations() {
    let control: AnnotationFeatureControl = serde_json::from_value(legacy_annotation()).unwrap();
    let tag: AnnotationFeatureTag = serde_json::from_value(legacy_annotation()).unwrap();
    for value in [
        serde_json::to_value(control).unwrap(),
        serde_json::to_value(tag).unwrap(),
    ] {
        for key in ["text_above", "text_below", "text_left", "text_right"] {
            assert!(value.get(key).is_none(), "omitted field {key} must remain absent");
        }
    }
}

#[test]
fn supplemental_text_round_trips_without_replacing_the_dimension() {
    let mut value = legacy_annotation();
    value["dimension"] = json!({ "symbol": "diameter", "dimension": 10.0, "tolerance": 0.05 });
    value["text_above"] = json!("Upper\nsecond line");
    value["text_below"] = json!("REFERENCE");
    value["text_left"] = json!("4X");
    value["text_right"] = json!("THRU \u{2300}");
    let control: AnnotationFeatureControl = serde_json::from_value(value.clone()).unwrap();
    let tag: AnnotationFeatureTag = serde_json::from_value(value.clone()).unwrap();
    let control = serde_json::to_value(control).unwrap();
    assert_eq!(control["dimension"], value["dimension"]);
    for encoded in [control, serde_json::to_value(tag).unwrap()] {
        for key in ["text_above", "text_below", "text_left", "text_right"] {
            assert_eq!(encoded[key], value[key]);
        }
    }
}
