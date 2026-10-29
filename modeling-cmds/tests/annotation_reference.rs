//! Regression coverage for annotation query and selection payloads.

#[cfg(test)]
mod tests {
    use kittycad_modeling_cmds::{
        each_cmd::SelectEntity,
        ok_response::OkModelingCmdResponse,
        shared::{EntityReference, EntityType},
    };
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn annotation_query_responses_can_be_reused_for_selection() {
        let annotation_id = Uuid::parse_str("e00e9caf-6e9e-5221-8545-b199546c367f").unwrap();
        let reference = json!({"type": "annotation", "annotation_id": annotation_id});
        for query in ["query_entity_type", "query_entity_type_with_point"] {
            let response: OkModelingCmdResponse = serde_json::from_value(json!({
                "type": query,
                "data": {"reference": reference}
            }))
            .unwrap();
            let returned_reference = match response {
                OkModelingCmdResponse::QueryEntityType(response) => response.reference,
                OkModelingCmdResponse::QueryEntityTypeWithPoint(response) => response.reference.unwrap(),
                other => panic!("unexpected response: {other:?}"),
            };
            assert_eq!(returned_reference, EntityReference::Annotation { annotation_id });

            let command = SelectEntity::builder().entities(vec![returned_reference]).build();
            let selection: SelectEntity = serde_json::from_value(serde_json::to_value(command).unwrap()).unwrap();
            assert_eq!(selection.entities, vec![EntityReference::Annotation { annotation_id }]);
        }
        assert_eq!(serde_json::to_value(EntityType::Annotation).unwrap(), "annotation");
    }
}
