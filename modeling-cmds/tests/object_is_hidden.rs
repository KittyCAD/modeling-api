//! Checks the wire format for object visibility queries.

use kittycad_modeling_cmds::{
    each_cmd::ObjectIsHidden, ok_response::OkModelingCmdResponse, shared::HiddenState, ModelingCmd,
};
use serde_json::json;
use uuid::Uuid;

#[test]
fn object_is_hidden_wire_format() {
    let object_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::nil()];
    let cmd = ModelingCmd::from(ObjectIsHidden::builder().object_ids(object_ids.clone()).build());
    assert!(!cmd.is_safe_to_batch());
    assert_eq!(
        serde_json::to_value(cmd).unwrap(),
        json!({ "type": "object_is_hidden", "object_ids": object_ids })
    );

    let expected = json!({
        "type": "object_is_hidden",
        "data": { "hidden": ["hidden", "visible", "no_such_object"] }
    });
    let response: OkModelingCmdResponse = serde_json::from_value(expected.clone()).unwrap();
    assert!(matches!(&response, OkModelingCmdResponse::ObjectIsHidden(data)
        if data.hidden == vec![HiddenState::Hidden, HiddenState::Visible, HiddenState::NoSuchObject]));
    assert_eq!(serde_json::to_value(response).unwrap(), expected);
}
