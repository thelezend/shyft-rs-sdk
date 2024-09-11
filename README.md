# Shyft Rust SDK

[![Crates.io](https://img.shields.io/crates/v/shyft-rs-sdk.svg)](https://crates.io/crates/shyft-rs-sdk)
[![CI](https://github.com/thelezend/shyft-rs-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/thelezend/shyft-rs-sdk/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/shyft-rs-sdk/badge.svg)](https://docs.rs/shyft-rs-sdk/)

Shyft SDK is a Rust library for interacting with the [Shyft API](https://docs.shyft.to/).

> **IMPORTANT: This is not an official SDK. It is a personal project for learning Rust and API development. The included endpoints are primarily for personal use in my own projects and may not cover all features of the Shyft API. However, new endpoints can be added upon request. Contributions are welcome!**

[Docs](https://docs.rs/shyft-rs-sdk/) | [Examples](https://github.com/thelezend/shyft-rs-sdk/tree/main/examples)

## Features

- Configurable retry strategy for API requests.
- Fetch transaction history for a given account. ([GET /transaction/history](https://docs.shyft.to/solana-apis/transactions/transaction-apis#get-transaction-history))
- Retrieve parsed transaction details for a specific transaction signature. ([GET /transaction/parsed](https://docs.shyft.to/solana-apis/transactions/transaction-apis#parsed-transaction))
- Fetch parsed bulk transactions in a single call. ([POST /transaction/parse_selected](https://docs.shyft.to/solana-apis/transactions/transaction-apis#post-transaction-parse_selected))

## Installation

Run the following command in your project directory:

```bash
cargo add shyft-rs-sdk
```

## Usage

The [examples](https://github.com/thelezend/shyft-rs-sdk/tree/main/examples) folder contains various examples of how to use the Shyft Rust SDK. The [docs](https://docs.rs/shyft-rs-sdk) also provide lots of code snippets and examples.

## Contribution

Your contributions are welcome! Please ensure your code follows the existing style and includes documentation and tests for any new functionality.

## License

This project is licensed under the MIT OR Apache-2.0 license.
