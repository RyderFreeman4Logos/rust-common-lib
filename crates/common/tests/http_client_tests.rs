
use common_core::http_client::*;
use common_core::prelude::*;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct MockData {
    foo: String,
    bar: u32,
}

#[tokio::test]
#[cfg(not(target_arch = "wasm32"))]
async fn test_request_builder_ext_take_data_success() {
    // Arrange
    let server = MockServer::start().await;
    let mock_data = MockData { foo: "hello".to_string(), bar: 123 };
    let response = ResponseTemplate::new(200).set_body_json(&mock_data);

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(response)
        .mount(&server)
        .await;

    let onion_client = OnionClientBuilder::default().retry(0u32).build().unwrap();
    let client: ClientWithMiddleware = AResult::from(onion_client).unwrap();
    let url = format!("{}{}", server.uri(), "/test");

    // Act
    let result: MockData = client.get(&url).take_data().await.unwrap();

    // Assert
    assert_eq!(result, mock_data);
}

#[tokio::test]
#[cfg(not(target_arch = "wasm32"))]
async fn test_request_builder_ext_take_data_http_error() {
    // Arrange
    let server = MockServer::start().await;
    let response = ResponseTemplate::new(500).set_body_string("Internal Server Error");

    Mock::given(method("GET"))
        .and(path("/error"))
        .respond_with(response)
        .mount(&server)
        .await;

    let onion_client = OnionClientBuilder::default().retry(0u32).build().unwrap();
    let client: ClientWithMiddleware = AResult::from(onion_client).unwrap();
    let url = format!("{}{}", server.uri(), "/error");

    // Act
    let result: AResult<MockData> = client.get(&url).take_data().await;

    // Assert
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("Internal Server Error"));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_onion_client_builder_and_conversion() {
    let api_key = "test_api_key".to_string();
    let client_result: AResult<ClientWithMiddleware> = OnionClientBuilder::default()
        .retry(5u32)
        .timeout(1000u64)
        .max_retry_interval(10000u64)
        .api_key(Some(api_key))
        .build()
        .unwrap()
        .into();

    assert!(client_result.is_ok());
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_onion_client_from_env() {
    // Test with env var set
    let key = "my-secret-key-from-env";
    std::env::set_var("API_KEY", key);
    let client_result = OnionClient::from_env();
    assert!(client_result.is_ok());
    // We can't easily inspect the headers of the final client,
    // but we can be reasonably sure it was built correctly.

    // Test with env var NOT set
    std::env::remove_var("API_KEY");
    let client_result_no_key = OnionClient::from_env();
    assert!(client_result_no_key.is_ok());
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_onion_client_with_api_key() {
    let api_key = "test_api_key_direct".to_string();
    let client_result = OnionClient::with_api_key(api_key);
    assert!(client_result.is_ok());
}
