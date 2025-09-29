use dropshot::ApiDescription;

use crate::websocket::WebSocketRequest;

#[tokio::test]
async fn openapi_lint() {
    let api = example_server().unwrap();
    // Create the API schema.
    let mut definition = api.openapi("Example Modeling API server", "1.2.3".parse().unwrap());
    let schema = definition
        .description("Example modeling API server")
        .contact_url("https://zoo.dev")
        .contact_email("api@zoo.dev")
        .json()
        .unwrap();

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

    /// Update the current value of the counter.  Note that the special value of 10
    /// is not allowed (just to demonstrate how to generate an error).
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
