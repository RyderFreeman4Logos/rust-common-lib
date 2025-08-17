use crate::prelude::*;
pub use reqwest::{self, header::HeaderMap, Client, RequestBuilder, Response};
pub use url::Url;

// Platform-specific imports and type aliases
cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        // Native-specific imports
        pub use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
        pub use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
        pub use reqwest_tracing::TracingMiddleware;
        use tokio::time::Duration;

        // Type alias for the native client
        pub type RequestClient = ClientWithMiddleware;
    } else {
        // Type alias for the Wasm client (which is just the base reqwest client)
        pub type RequestClient = Client;
    }
}

// Common trait for both platforms
#[async_trait::async_trait(?Send)]
pub trait RequestBuilderExt {
    async fn take_data<T>(self) -> AResult<T>
    where
        T: serde::de::DeserializeOwned;
}

// Implement the trait for the base reqwest::RequestBuilder, which is used in Wasm
#[async_trait::async_trait(?Send)]
impl RequestBuilderExt for RequestBuilder {
    async fn take_data<T>(self) -> AResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response: Response = self.send().await?;
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            bail!("{:#?}", response.text().await?);
        } else {
            let result: T = response.json().await?;
            Ok(result)
        }
    }
}

// Native-only implementation for the middleware-equipped RequestBuilder
#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait(?Send)]
impl RequestBuilderExt for reqwest_middleware::RequestBuilder {
    async fn take_data<T>(self) -> AResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response: Response = self.send().await?;
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            bail!("{:#?}", response.text().await?);
        } else {
            let result: T = response.json().await?;
            Ok(result)
        }
    }
}

// Native-only builder and implementation for the middleware-equipped client
#[cfg(not(target_arch = "wasm32"))]
#[derive(Builder)]
#[builder(setter(into))]
pub struct OnionClient {
    #[builder(default = "3")]
    retry: u32,
    #[builder(default = "60_000")]
    timeout: u64,
    #[builder(default = "600_000")]
    max_retry_interval: u64,
    #[builder(default = "None")]
    api_key: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl OnionClient {
    pub fn from_env() -> AResult<ClientWithMiddleware> {
        let api_key = std::env::var("API_KEY").ok();
        OnionClientBuilder::default()
            .retry(0u32)
            .api_key(api_key)
            .build()?
            .into()
    }

    pub fn with_api_key(api_key: String) -> AResult<ClientWithMiddleware> {
        OnionClientBuilder::default()
            .retry(0u32)
            .api_key(Some(api_key))
            .build()?
            .into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<OnionClient> for AResult<ClientWithMiddleware> {
    fn from(config: OnionClient) -> Self {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                Duration::from_millis(config.timeout),
                Duration::from_millis(config.max_retry_interval),
            )
            .build_with_max_retries(config.retry);

        let mut builder = Client::builder();

        if let Some(api_key) = config.api_key {
            let mut headers = HeaderMap::new();
            let value = format!("Bearer {api_key}").parse()?;
            headers.insert("Authorization", value);
            builder = builder.default_headers(headers);
        }

        let client = builder.build().map_err(msg)?;
        let client_with_middleware = ClientBuilder::new(client)
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(client_with_middleware)
    }
}
