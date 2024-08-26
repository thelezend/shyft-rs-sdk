mod common;

use shyft_rs_sdk::api::ShyftApi;

#[tokio::test]
async fn test_get_transaction_history() {
    common::setup();

    let api_key = std::env::var("SHYFT_API_KEY").expect("SHYFT_API_KEY must be set");

    // Sample account and transaction signature
    let test_account = "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna";
    let before_tx_signature =
        "uZz2qDvLzndsTEY31YkxgRe1rYQ8MsCtS3DwhPYP1Md7u7dnUK4HW3vYzsxE6GSxFhSG5zpvqSQnSUn1sPmzTBu";

    // Create ShyftApi instance
    let shyft_api = ShyftApi::new(&api_key, None, None, None).expect("Failed to create ShyftApi");

    // Fetch transaction history
    let transaction_history = shyft_api
        .get_transaction_history(test_account, Some(1), Some(before_tx_signature))
        .await;

    // Assert that the transaction history is fetched successfully
    assert!(
        transaction_history.is_ok(),
        "Failed to fetch transaction history: {:?}",
        transaction_history.err()
    );

    // Assert that the transaction history is not empty
    assert!(
        !transaction_history.unwrap().is_empty(),
        "Transaction history is empty"
    );
}
