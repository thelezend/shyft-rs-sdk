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
- Fetch parsed bulk transactions in a single call.

## Configuration

You can configure the retry strategy by providing optional parameters when creating the [`ShyftApi`] instance:

- `min_retry_interval`: Minimum retry interval in milliseconds.
- `max_retry_interval`: Maximum retry interval in milliseconds.
- `max_retries`: Maximum number of retries.
- `network`: Network to interact with(mainnet-beta, devnet, testnet).
- `commitment`: Commitment level for transactions(confirmed, finalised).

## Usage

### Creating a ShyftApi Instance

```
use shyft_rs_sdk::ShyftApi;

let api_key = "your_api_key";
let client = ShyftApi::new(api_key, None, None, None, None, None)
    .expect("Failed to create ShyftApi");
```

### Fetching Transaction History

Equivalent to [GET /transaction/history]

[GET /transaction/history]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#get-transaction-history

```no_run
# #[tokio::main]
# async fn main() -> Result<(), shyft_rs_sdk::Error> {
# use shyft_rs_sdk::ShyftApi;
#
# let api_key = "your_api_key";
# let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
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
# let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
let tx_signature = "your_transaction_signature";
let parsed_transaction_details = client
    .get_transaction_parsed(tx_signature)
    .await
    .expect("Failed to fetch parsed transaction details");
println!("{:?}", parsed_transaction_details);
# Ok(())
# }
```

### Fetching Parsed Bulk Transactions

Equivalent to [POST /transaction/parse_selected]

[POST /transaction/parse_selected]: https://docs.shyft.to/solana-apis/transactions/transaction-apis#post-transaction-parse_selected

```no_run
# #[tokio::main]
# async fn main() -> Result<(), shyft_rs_sdk::Error> {
# use shyft_rs_sdk::ShyftApi;
#
# let api_key = "your_api_key";
# let client = ShyftApi::new(api_key, None, None, None, None, None).unwrap();
let tx_signatures = vec![
    "your_transaction_signature_1".to_owned(),
    "your_transaction_signature_2".to_owned(),
];
let parsed_transactions = client
    .get_transaction_parse_selected(&tx_signatures, Some(true), Some(true))
    .await
    .expect("Failed to fetch parsed bulk transactions");
println!("{:?}", parsed_transactions);
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

pub use api::{Commitment, Network, ShyftApi};
pub use error::Error;
