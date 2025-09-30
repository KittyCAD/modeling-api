use dropshot::ApiDescription;

use crate::websocket::WebSocketRequest;

#[tokio::test]
async fn test_openapi() {
    let api = example_server().unwrap();
    // Create the API schema.
    let mut definition = api.openapi("Example Modeling API server", "1.2.3".parse().unwrap());
    let schema = definition
        .description("Example modeling API server")
        .contact_url("https://zoo.dev")
        .contact_email("api@zoo.dev")
        .json()
        .unwrap();
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();
    expectorate::assert_contents("openapi/api.json", &schema_str);

    let spec: openapiv3::OpenAPI = serde_json::from_value(schema).expect("schema was not valid OpenAPI");

    assert_eq!(spec.openapi, "3.0.3");

    // Check for lint errors.
    let errors = openapi_lint::validate(&spec);
    assert!(errors.is_empty(), "{}", errors.join("\n\n"));
}

fn example_server() -> Result<ApiDescription<()>, String> {
    use dropshot::endpoint;
    use dropshot::ApiDescription;
    use dropshot::HttpError;
    use dropshot::HttpResponseUpdatedNoContent;
    use dropshot::RequestContext;
    use dropshot::TypedBody;

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

async fn get_current_schema() -> openapiv3::Schema {
    let branch = "main";
    let file = "openapi/api.json";
    let path =
        format!("https://raw.githubusercontent.com/KittyCAD/modeling-api/refs/heads/{branch}/modeling-cmds/{file}");
    dbg!(&path);
    reqwest::get(path).await.unwrap().json().await.unwrap()
}

#[tokio::test]
async fn test_schema() {
    let s = get_current_schema().await;
}
