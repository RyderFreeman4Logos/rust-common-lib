use crate::prelude::*;
// use crate::prelude::{
//     AResult,
//     msg,
//     Builder,
// };


// pub use wasm_bindgen;

pub use url::Url;

pub use reqwest_middleware::{
    ClientBuilder,
    ClientWithMiddleware,
};
pub use reqwest_retry::{
    RetryTransientMiddleware,
    policies::ExponentialBackoff
};
pub use reqwest_tracing::TracingMiddleware;
pub use reqwest::{
    self,
    // ClientBuilder,
    Response,
    RequestBuilder,
    Client,
    // Proxy,
    header::HeaderMap
};

cfg_if!{
    if #[cfg(target_arch = "wasm32")] {
        pub type RequestClient = reqwest::Client;
        pub type HttpRequestBuilder = RequestBuilder;
    } else {
        pub type RequestClient = ClientWithMiddleware;
        pub type HttpRequestBuilder = reqwest_middleware::RequestBuilder;
    }
}

#[async_trait::async_trait(?Send)]
pub trait RequestBuilderExt {
    async fn take_data<T>(self) -> AResult<T>
    where
        T: serde::de::DeserializeOwned;
}

#[async_trait::async_trait(?Send)]
impl RequestBuilderExt for HttpRequestBuilder {
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



use tokio::time::Duration;

#[derive(Builder)]
#[builder(setter(into))]
// #[builder(setter(into, optional), build_fn(into))]
pub struct OnionClient {
    // #[builder(default = "true")]
    // zstd: bool,
    #[builder(default = "3")]
    retry: u32,
    #[builder(default = "60_000")]
    timeout: u64,
    #[builder(default = "600_000")]
    max_retry_interval: u64,
    #[builder(default = "None")]
    api_key: Option<String>,
}


impl OnionClient {
    pub fn try_new() -> AResult<ClientWithMiddleware> {
        let api_key = std::env::var("API_KEY").unwrap_or("eUBC6atrAXG9hXFvSvJCAhBJG3PuhpynKPvFpXQxSS54H2LuJvawmW8LvsngzeRcDQ6sqYK4dS9KFNFK732CMexgLRDAY5A72rAmHhqem5RkDcJh4jW6YT2e8ZmqHN4K".to_string());
        OnionClientBuilder::default()
            .retry(0u32)
            .api_key(Some(api_key))
            // .proxy(Proxy::all("socks5://127.0.0.1:9050")?)
            .build()?
            .into()
    }
}


impl From<OnionClient> for AResult<ClientWithMiddleware> {
    fn from(config: OnionClient) -> Self {
        // info!("retry_policy");
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                Duration::from_millis(config.timeout),
                Duration::from_millis(config.max_retry_interval)
            )
            .build_with_max_retries(config.retry);

        let mut builder = Client::builder();
        // .zstd(config.zstd)
        // .timeout(retry_policy.min_retry_interval);


        if let Some(api_key) = config.api_key {
            let mut headers = HeaderMap::new();
            let value = format!("Bearer {api_key}")
                .parse()?;
            headers.insert("Authorization", value);
            builder = builder.default_headers(headers);
        }

        let client = builder.build()
            .map_err(msg)?;
        let client_with_middleware = ClientBuilder::new(client)
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(TracingMiddleware::default())
            // Retry failed requests.
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(client_with_middleware)
    }
}

