use mockito::{Server, ServerGuard};
use serde::{Deserialize, Serialize};
use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

#[derive(Serialize, Debug)]
struct HelloRequest {
    name: String,
}

impl ServiceStackRequest for HelloRequest {
    type Response = HelloResponse;

    fn path(&self) -> String {
        "/hello".to_string()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct HelloResponse {
    result: String,
}

impl ServiceStackResponse for HelloResponse {}

#[tokio::test]
async fn test_post_request() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/hello")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":"Hello, World!"}"#)
        .create_async()
        .await;

    let client = JsonServiceClient::new(server.url());
    let request = HelloRequest {
        name: "World".to_string(),
    };

    let response: HelloResponse = client.post(request).await.unwrap();
    assert_eq!(response.result, "Hello, World!");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_request() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/hello")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":"Hello, World!"}"#)
        .create_async()
        .await;

    let client = JsonServiceClient::new(server.url());
    let request = HelloRequest {
        name: "World".to_string(),
    };

    let response: HelloResponse = client.get(request).await.unwrap();
    assert_eq!(response.result, "Hello, World!");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_bearer_token_authentication() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/hello")
        .match_header("Authorization", "Bearer test-token-123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":"Authenticated!"}"#)
        .create_async()
        .await;

    let mut client = JsonServiceClient::new(server.url());
    client.set_bearer_token("test-token-123");

    let request = HelloRequest {
        name: "World".to_string(),
    };

    let response: HelloResponse = client.post(request).await.unwrap();
    assert_eq!(response.result, "Authenticated!");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/hello")
        .with_status(404)
        .with_body("Not Found")
        .create_async()
        .await;

    let client = JsonServiceClient::new(server.url());
    let request = HelloRequest {
        name: "World".to_string(),
    };

    let result = client.post(request).await;
    assert!(result.is_err());

    if let Err(e) = result {
        match e {
            servicestack::ServiceStackError::ApiError { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            _ => panic!("Expected ApiError"),
        }
    }
    mock.assert_async().await;
}

#[derive(Serialize, Debug)]
struct SearchRequest {
    query: String,
    limit: u32,
}

impl ServiceStackRequest for SearchRequest {
    type Response = SearchResponse;

    fn path(&self) -> String {
        "/search".to_string()
    }

    fn method(&self) -> servicestack::HttpMethod {
        servicestack::HttpMethod::Put
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct SearchResponse {
    results: Vec<String>,
}

impl ServiceStackResponse for SearchResponse {}

#[tokio::test]
async fn test_custom_http_method() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/search")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":["result1","result2"]}"#)
        .create_async()
        .await;

    let client = JsonServiceClient::new(server.url());
    let request = SearchRequest {
        query: "test".to_string(),
        limit: 10,
    };

    let response: SearchResponse = client.send(request).await.unwrap();
    assert_eq!(response.results, vec!["result1", "result2"]);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_raw_request_method() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/custom")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":"Custom response"}"#)
        .create_async()
        .await;

    let client = JsonServiceClient::new(server.url());

    #[derive(Serialize)]
    struct CustomRequest {
        data: String,
    }

    #[derive(Deserialize)]
    struct CustomResponse {
        result: String,
    }

    let request = CustomRequest {
        data: "test".to_string(),
    };

    let response: CustomResponse = client
        .request("POST", "/custom", Some(&request))
        .await
        .unwrap();

    assert_eq!(response.result, "Custom response");
    mock.assert_async().await;
}
