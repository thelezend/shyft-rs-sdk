mod transactions;

use std::{collections::HashMap, time::Duration};

use reqwest::header;

use crate::{constants, reqwest_ext::get_retry_strategy};

/// Struct representing the Shyft API client
#[derive(Debug, Clone)]
pub struct ShyftApi {
    client: reqwest_middleware::ClientWithMiddleware,
    default_params: HashMap<String, String>,
}

impl ShyftApi {
    /// Creates a new instance of `ShyftApi`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice holding the API key.
    /// * `min_retry_interval` - An optional minimum retry interval in milliseconds.
    /// * `max_retry_interval` - An optional maximum retry interval in milliseconds.
    /// * `max_retries` - An optional maximum number of retries.
    ///
    /// # Returns
    ///
    /// * `Result<Self, crate::error::Error>` - A result containing the `ShyftApi` instance or an error.
    pub fn new(
        api_key: &str,
        min_retry_interval: Option<u64>,
        max_retry_interval: Option<u64>,
        max_retries: Option<u32>,
    ) -> Result<Self, crate::error::Error> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::from_str(api_key).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("x-api-key", auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()?;

        let mut default_params = HashMap::new();
        default_params.insert("network".to_string(), "mainnet-beta".to_string());

        let retry_s = get_retry_strategy(
            min_retry_interval.unwrap_or(constants::MIN_RETRY_INTERVAL),
            max_retry_interval.unwrap_or(constants::MAX_RETRY_INTERVAL),
            max_retries.unwrap_or(constants::MAX_RETRIES),
        );

        let client = reqwest_middleware::ClientBuilder::new(client)
            .with(retry_s)
            .build();

        Ok(Self {
            client,
            default_params,
        })
    }
}
