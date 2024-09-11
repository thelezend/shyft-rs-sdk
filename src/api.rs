//! Shyft API Client
//!
//! This module provides a client for interacting with the Shyft API.
//!
//! # Examples
//!
//! ```
//! use shyft_rs_sdk::ShyftApi;
//!
//! let api_key = "your_api_key";
//! let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
//! ```
//!
//! The [`ShyftApi`] struct allows you to create a new client instance and interact with the Shyft API.
//! You can customize the retry strategy by providing optional parameters for minimum retry interval,
//! maximum retry interval, and maximum number of retries.

use std::{collections::HashMap, time::Duration};

use reqwest::header;

use crate::{
    constants,
    models::{self, parsed_transaction_details::ParsedTransactionDetails},
    reqwest_ext::get_retry_strategy,
};

#[derive(Debug, Clone)]
/// Struct representing the Shyft API client
///
/// This struct allows you to create a new client instance and interact with the Shyft API.
/// It includes a [`ClientWithMiddleware`] for handling HTTP requests
/// and a [`HashMap`](std::collections::HashMap) for storing default parameters.
///
/// [`ClientWithMiddleware`]: https://docs.rs/reqwest-middleware/latest/reqwest_middleware/struct.ClientWithMiddleware.html
pub struct ShyftApi {
    /// The HTTP client with middleware for handling requests and retries.
    client: reqwest_middleware::ClientWithMiddleware,
    /// A map of default parameters to be included in each API request.
    default_params: HashMap<String, String>,
}

/// Enum representing different Shyft API networks.
///
/// This enum is used to specify the network to interact with when making API requests.
#[derive(strum_macros::Display, Debug, Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum Network {
    /// Mainnet Beta network
    MainnetBeta,
    /// Devnet network
    Devnet,
    /// Testnet network
    Testnet,
}

#[derive(strum_macros::Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
/// Enum representing the commitment level for transactions.
///
/// This enum is used to specify the commitment level when making API requests.
pub enum Commitment {
    /// The transaction has been confirmed by the cluster.
    Confirmed,
    /// The transaction has been finalized and will not be rolled back.
    Finalized,
}

impl ShyftApi {
    /// Creates a new instance of the Shyft API client.
    ///
    /// This function initializes a new Shyft API client with the provided API key and optional parameters
    /// for retry strategy, network, and commitment level.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.
    /// * `min_retry_interval` - An optional minimum retry interval in seconds.
    /// * `max_retry_interval` - An optional maximum retry interval in seconds.
    /// * `max_retries` - An optional maximum number of retries.
    /// * `network` - An optional network to interact with.
    /// * `commitment` - An optional commitment level for transactions.
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP client could not be built.
    ///
    /// # Examples
    ///
    /// ```
    /// use shyft_rs_sdk::ShyftApi;
    ///
    /// let api_key = "your_api_key";
    /// let client = ShyftApi::new(api_key, None, None, None, None, None)?;
    /// # Ok::<(), shyft_rs_sdk::Error>(())
    /// ```
    ///
    /// With custom parameters:
    /// ```
    /// # use shyft_rs_sdk::{ShyftApi, Network, Commitment};
    /// # let api_key = "your_api_key";
    /// let min_retry_interval = Some(1);
    /// let max_retry_interval = Some(10);
    /// let max_retries = Some(5);
    /// let network = Some(Network::MainnetBeta);
    /// let commitment = Some(Commitment::Confirmed);
    /// let client = ShyftApi::new(api_key, min_retry_interval, max_retry_interval, max_retries, network, commitment)?;
    /// # Ok::<(), shyft_rs_sdk::Error>(())
    /// ```
    pub fn new(
        api_key: &str,
        min_retry_interval: Option<u64>,
        max_retry_interval: Option<u64>,
        max_retries: Option<u32>,
        network: Option<Network>,
        commitment: Option<Commitment>,
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
        default_params.insert(
            "network".to_string(),
            network.unwrap_or(Network::MainnetBeta).to_string(),
        );
        default_params.insert(
            "commitment".to_string(),
            commitment.unwrap_or(Commitment::Confirmed).to_string(),
        );

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

    /// Fetches the transaction history for a given account. Equivalent to [GET /transaction/history]
    ///
    /// [GET /transaction/history]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#get-transaction-history
    ///
    /// # Arguments
    ///
    /// * `account` - The account for which to fetch the transaction history.
    /// * `tx_num` - An optional number of transactions to fetch.
    /// * `before_tx_signature` - An optional transaction signature to fetch transactions before.
    /// * `until_tx_signature` - An optional transaction signature to fetch transactions until.
    /// * `enable_raw` - An optional flag to enable raw transaction details.
    /// * `enable_events` - An optional flag to enable events in the transaction details.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or the response status is not 200.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), shyft_rs_sdk::Error> {
    /// # use shyft_rs_sdk::ShyftApi;
    /// #
    /// # let api_key = "your_api_key";
    /// # let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
    /// let history = client.get_transaction_history("account_address", Some(10), None, None, Some(true), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction_history(
        &self,
        account: &str,
        tx_num: Option<u32>,
        before_tx_signature: Option<&str>,
        until_tx_signature: Option<&str>,
        enable_raw: Option<bool>,
        enable_events: Option<bool>,
    ) -> Result<Vec<ParsedTransactionDetails>, crate::error::Error> {
        let url = format!("{}transaction/history", constants::URL);

        let mut request = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("account", account)]);

        if let Some(tx_num) = tx_num {
            request = request.query(&[("tx_num", tx_num.to_string())]);
        }

        if let Some(before_tx_signature) = before_tx_signature {
            request = request.query(&[("before_tx_signature", before_tx_signature)]);
        }

        if let Some(until_tx_signature) = until_tx_signature {
            request = request.query(&[("until_tx_signature", until_tx_signature)]);
        }

        if let Some(enable_raw) = enable_raw {
            request = request.query(&[("enable_raw", enable_raw.to_string())]);
        }

        if let Some(enable_events) = enable_events {
            request = request.query(&[("enable_events", enable_events.to_string())]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<Vec<ParsedTransactionDetails>>>()
            .await?
            .result;
        Ok(parsed_response)
    }

    /// Retrieves parsed transaction details for a given transaction signature. Equivalent to [GET /transaction/parsed]
    ///
    /// [GET /transaction/parsed]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#parsed-transaction
    ///
    /// # Arguments
    ///
    /// * `tx_signature` - A string slice that holds the transaction signature.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or the response status is not 200.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), shyft_rs_sdk::Error> {
    /// # use shyft_rs_sdk::ShyftApi;
    /// #
    /// # let api_key = "your_api_key";
    /// # let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
    /// let transaction_details = client.get_transaction_parsed("transaction_signature").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction_parsed(
        &self,
        tx_signature: &str,
    ) -> Result<ParsedTransactionDetails, crate::error::Error> {
        let url = format!("{}transaction/parsed", constants::URL);
        let response = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("txn_signature", tx_signature)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<ParsedTransactionDetails>>()
            .await?
            .result;
        Ok(parsed_response)
    }

    /// Retrieves parsed transaction details for given transaction signatures. Equivalent to [POST /transaction/parse_selected]
    ///
    /// [POST /transaction/parse_selected]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#post-transaction-parse_selected
    ///
    /// # Arguments
    ///
    /// * `transaction_signatures` - A slice of strings that holds the transaction signatures.
    /// * `enable_raw` - An optional boolean to include raw transaction details.
    /// * `enable_events` - An optional boolean to include events in the transaction details.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or the response status is not 200.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), shyft_rs_sdk::Error> {
    /// # use shyft_rs_sdk::ShyftApi;
    /// #
    /// # let api_key = "your_api_key";
    /// # let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
    /// let transaction_signatures = vec!["transaction_signature1".to_string(), "transaction_signature2".to_string()];
    /// let transaction_details = client.get_transaction_parse_selected(&transaction_signatures, Some(true), Some(true)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction_parse_selected(
        &self,
        transaction_signatures: &[String],
        enable_raw: Option<bool>,
        enable_events: Option<bool>,
    ) -> Result<Vec<ParsedTransactionDetails>, crate::error::Error> {
        let url = format!("{}transaction/parse_selected", constants::URL);
        let request = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&serde_json::json!({
                    "network": self.default_params.get("network").unwrap(),
                    "transaction_signatures": transaction_signatures,
                    "enable_raw": enable_raw.unwrap_or(false),
                    "enable_events": enable_events.unwrap_or(false),
                    "commitment": self.default_params.get("commitment").unwrap(),
                }))
                .unwrap(),
            );

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<Vec<ParsedTransactionDetails>>>()
            .await?
            .result;
        Ok(parsed_response)
    }
}
