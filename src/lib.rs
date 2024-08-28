/*!
# Shyft Rust SDK

Shyft SDK is a Rust library for interacting with the [Shyft API](https://docs.shyft.to/).

<div class="warning">
This is not an official SDK. It is a personal project for learning Rust and API development. The included endpoints are primarily for personal use in my own projects and may not cover all features of the Shyft API.
</div>

## Features

- Configurable retry strategy for API requests.
- Fetch transaction history for a given account.
- Retrieve parsed transaction details for a specific transaction signature.

## Configuration

You can configure the retry strategy by providing optional parameters when creating the [`ShyftApi`] instance:

- `min_retry_interval`: Minimum retry interval in milliseconds.
- `max_retry_interval`: Maximum retry interval in milliseconds.
- `max_retries`: Maximum number of retries.

## Usage

### Creating a ShyftApi Instance

```
use shyft_rs_sdk::ShyftApi;

let api_key = "your_api_key";
let client = ShyftApi::new(api_key, None, None, None)
    .expect("Failed to create ShyftApi");
```

### Fetching Transaction History



```no_run
# #[tokio::main]
# async fn main() -> Result<(), shyft_rs_sdk::Error> {
# use shyft_rs_sdk::ShyftApi;
#
# let api_key = "your_api_key";
# let client = ShyftApi::new(api_key, None, None, None).unwrap();
let account = "your_account_address";
let transaction_history = client
    .get_transaction_history(account, Some(10), None, None, Some(true), None)
    .await?;
println!("{:?}", transaction_history);
# Ok(())
# }
```

### Fetching Parsed Transaction Details

Equivalent to [GET /transaction/parsed]

[GET /transaction/parsed]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#parsed-transaction

```no_run
# #[tokio::main]
# async fn main() -> Result<(), shyft_rs_sdk::Error> {
# use shyft_rs_sdk::ShyftApi;
#
# let api_key = "your_api_key";
# let client = ShyftApi::new(api_key, None, None, None).unwrap();
let tx_signature = "your_transaction_signature";
let parsed_transaction_details = client
    .get_transaction_parsed(tx_signature)
    .await
    .expect("Failed to fetch parsed transaction details");
println!("{:?}", parsed_transaction_details);
# Ok(())
# }
```
 */

#![warn(
    missing_docs,
    rustdoc::unescaped_backticks,
    clippy::missing_errors_doc,
    clippy::missing_docs_in_private_items
)]

mod api;
mod constants;
mod error;
mod reqwest_ext;

pub mod models;

pub use api::ShyftApi;
pub use error::Error;
