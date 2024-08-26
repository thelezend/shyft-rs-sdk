use std::{collections::HashMap, time::Duration};

use reqwest::header;

use crate::{
    constants,
    reqwest_ext::{self, get_retry_strategy},
};

use super::models::{self, parsed_transaction_details::ParsedTransactionDetails};

/// Struct representing the Shyft API client
#[derive(Debug, Clone)]
pub struct ShyftApi {
    client: reqwest_middleware::ClientWithMiddleware,
    default_params: HashMap<String, String>,
}

impl ShyftApi {
    /// Creates a new instance of ShyftApi
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.
    /// * `min_retry_interval` - An optional minimum retry interval in milliseconds.
    /// * `max_retry_interval` - An optional maximum retry interval in milliseconds.
    /// * `max_retries` - An optional maximum number of retries.
    ///
    /// # Returns
    ///
    /// * `Result<Self, reqwest_ext::Error>` - A result containing the ShyftApi instance or an error.
    pub fn new(
        api_key: &str,
        min_retry_interval: Option<u64>,
        max_retry_interval: Option<u64>,
        max_retries: Option<u32>,
    ) -> Result<Self, reqwest_ext::Error> {
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

    /// Fetches the transaction history for a given account.
    ///
    /// # Arguments
    ///
    /// * `account` - A string slice representing the account address.
    /// * `tx_num` - An optional number of transactions to fetch.
    /// * `before_tx_signature` - An optional transaction signature to fetch transactions before.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ParsedTransactionDetails>, reqwest_ext::Error>` - A result containing a vector of parsed transaction details or an error.
    pub async fn get_transaction_history(
        &self,
        account: &str,
        tx_num: Option<u32>,
        before_tx_signature: Option<&str>,
    ) -> Result<Vec<ParsedTransactionDetails>, reqwest_ext::Error> {
        let url = format!("{}transaction/history", constants::URL);

        let mut request = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("account", account)])
            .query(&[("enable_raw", "true")]);

        if let Some(tx_num) = tx_num {
            request = request.query(&[("tx_num", tx_num.to_string())]);
        }

        if let Some(before_tx_signature) = before_tx_signature {
            request = request.query(&[("before_tx_signature", before_tx_signature)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(reqwest_ext::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<Vec<ParsedTransactionDetails>>>()
            .await?
            .result;
        Ok(parsed_response)
    }

    /// Fetches the parsed transaction details for a given transaction signature.
    ///
    /// # Arguments
    ///
    /// * `tx_signature` - A string slice representing the transaction signature.
    ///
    /// # Returns
    ///
    /// * `Result<ParsedTransactionDetails, reqwest_ext::Error>` - A result containing the parsed transaction details or an error.
    pub async fn get_parsed_transaction_details(
        &self,
        tx_signature: &str,
    ) -> Result<ParsedTransactionDetails, reqwest_ext::Error> {
        let url = format!("{}transaction/parsed", constants::URL);
        let response = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("txn_signature", tx_signature)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(reqwest_ext::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<ParsedTransactionDetails>>()
            .await?
            .result;
        Ok(parsed_response)
    }
}
