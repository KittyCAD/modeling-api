use dropshot::ApiDescription;

use crate::websocket::WebSocketRequest;

#[tokio::test]
async fn test_openapi() {
    let api = example_server().unwrap();
    // Create the API schema.
    let mut definition = api.openapi("Example Modeling API server", "1.2.3".parse().unwrap());
    let mut schema = definition
        .description("Example modeling API server")
        .contact_url("https://zoo.dev")
        .contact_email("api@zoo.dev")
        .json()
        .unwrap();
    sort_json_keys(&mut schema);
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();
    expectorate::assert_contents("openapi/api.json", &schema_str);

    let spec: openapiv3::OpenAPI = serde_json::from_value(schema).expect("schema was not valid OpenAPI");

    assert_eq!(spec.openapi, "3.0.3");

    // Check for lint errors.
    let mut schema_to_lint = spec;
    exclude_kcl_version_from_lint(&mut schema_to_lint);
    let errors = openapi_lint::validate(&schema_to_lint);
    assert!(errors.is_empty(), "{}", errors.join("\n\n"));

    // Download the old schema, write it to disk.
    let schema = download_openapi_schema("main").await;
    std::fs::write("openapi/old_api.json", schema).unwrap();
}

/// KCL versions serialize like "2.0" or "3.0-preview", because that's how users
/// write them in KCL files. And I want the two representations (de/serialized form, and parsed/unparsed form)
/// to match. But this violates the `openapi_lint` conventions. So let's just exclude that from
/// the schema's linter.
fn exclude_kcl_version_from_lint(spec: &mut openapiv3::OpenAPI) {
    let Some(schema) = spec
        .components
        .as_mut()
        .expect("OpenAPI components are missing")
        .schemas
        .get_mut("KclVersion")
    else {
        return;
    };
    *schema = openapiv3::ReferenceOr::Item(openapiv3::Schema {
        schema_data: Default::default(),
        schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(Default::default())),
    });
}

#[test]
fn test_exclude_kcl_version_from_lint() {
    let schema = example_server()
        .unwrap()
        .openapi("Example Modeling API server", "1.2.3".parse().unwrap())
        .json()
        .unwrap();
    let mut spec: openapiv3::OpenAPI = serde_json::from_value(schema).unwrap();
    assert!(!openapi_lint::validate(&spec).is_empty());
    let mut expected = serde_json::to_value(&spec).unwrap();
    expected["components"]["schemas"]["KclVersion"] = serde_json::json!({ "type": "string" });

    exclude_kcl_version_from_lint(&mut spec);

    // All other schemas and references must remain unchanged.
    assert_eq!(serde_json::to_value(&spec).unwrap(), expected);
    let errors = openapi_lint::validate(&spec);
    assert!(errors.is_empty(), "{}", errors.join("\n\n"));
}

fn sort_json_keys(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                sort_json_keys(value);
                if matches!(key.as_str(), "properties" | "schemas") {
                    if let serde_json::Value::Object(map) = value {
                        map.sort_keys();
                    }
                }
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                sort_json_keys(value);
            }
        }
        _ => {}
    }
}

fn example_server() -> Result<ApiDescription<()>, String> {
    use dropshot::{endpoint, ApiDescription, HttpError, HttpResponseUpdatedNoContent, RequestContext, TypedBody};

    #[endpoint {
        method = PUT,
        path = "/example",
    }]
    async fn example(
        _: RequestContext<()>,
        _: TypedBody<WebSocketRequest>,
    ) -> Result<HttpResponseUpdatedNoContent, HttpError> {
        Ok(HttpResponseUpdatedNoContent())
    }

    // Build a description of the API.
    let mut api = ApiDescription::new();
    api.register(example).unwrap();

    Ok(api)
}

async fn download_openapi_schema(branch: &str) -> String {
    let file = "openapi/api.json";
    let path =
        format!("https://raw.githubusercontent.com/KittyCAD/modeling-api/refs/heads/{branch}/modeling-cmds/{file}");
    reqwest::get(path).await.unwrap().text().await.unwrap()
}
