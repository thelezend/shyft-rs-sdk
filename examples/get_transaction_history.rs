mod common;

use shyft_rs_sdk::ShyftApi;

#[tokio::main]
async fn main() {
    common::setup();

    let api_key = std::env::var("SHYFT_API_KEY").expect("SHYFT_API_KEY must be set");

    // Sample account and transaction signature
    let test_account = "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna";
    let before_tx_signature =
        "uZz2qDvLzndsTEY31YkxgRe1rYQ8MsCtS3DwhPYP1Md7u7dnUK4HW3vYzsxE6GSxFhSG5zpvqSQnSUn1sPmzTBu";

    // Create ShyftApi instance
    let shyft_api =
        ShyftApi::new(&api_key, None, None, None, None, None).expect("Failed to create ShyftApi");

    // Fetch transaction history
    let transaction_history = shyft_api
        .get_transaction_history(
            test_account,
            Some(1),
            Some(before_tx_signature),
            None,
            Some(true),
            None,
        )
        .await;

    println!("{:?}", transaction_history);
}
