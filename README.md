# Shyft SDK

Shyft SDK is a Rust library for interacting with the Shyft API, providing functionalities to fetch transaction history and parsed transaction details. New api requests will be added if needed.

## Features

- Fetch transaction history for a given account.
- Retrieve parsed transaction details for a specific transaction signature.
- Configurable retry strategy for API requests.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
shyft-sdk = { git = "https://github.com/thelezend/shyft-sdk.git", branch = "main"}
```

## Usage

### Creating a ShyftApi Instance

```rust
use shyft_sdk::api::ShyftApi;
let api_key = "your_api_key";
let shyft_api = ShyftApi::new(api_key, None, None, None).expect("Failed to create ShyftApi");
```

### Fetching Transaction History

```rust
let account = "your_account_address";
let transaction_history = shyft_api
.get_transaction_history(account, Some(10), None)
.await
.expect("Failed to fetch transaction history");
println!("{:?}", transaction_history);
```

### Fetching Parsed Transaction Details

```rust
let tx_signature = "your_transaction_signature";
let parsed_transaction_details = shyft_api
.get_parsed_transaction_details(tx_signature)
.await
.expect("Failed to fetch parsed transaction details");
println!("{:?}", parsed_transaction_details);
```

## Configuration

You can configure the retry strategy by providing optional parameters when creating the `ShyftApi` instance:

- `min_retry_interval`: Minimum retry interval in milliseconds.
- `max_retry_interval`: Maximum retry interval in milliseconds.
- `max_retries`: Maximum number of retries.

Example:

```rust
let shyft_api = ShyftApi::new(api_key, Some(500), Some(1000), Some(3)).expect("Failed to create ShyftApi");
```

## Running Tests

To run the tests, set the `SHYFT_API_KEY` environment variable and use `cargo test`:

```sh
export SHYFT_API_KEY=your_api_key
cargo test
```

## Contribution

Your contributions are welcome! To contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes.
4. Commit your changes (`git commit -m "feat: add your feature"`).
5. Push to the branch (`git push origin feature/your-feature`).
6. Create a pull request.

Please ensure your code follows the existing style and includes tests for any new functionality.

## License

This project is licensed under the MIT License.
