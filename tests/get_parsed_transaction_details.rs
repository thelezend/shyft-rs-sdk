mod common;

use shyft_rs_sdk::ShyftApi;

#[tokio::test]
async fn test_get_parsed_transaction_details() {
    common::setup();

    // Retrieve the API key from environment variables
    let api_key = std::env::var("SHYFT_API_KEY").expect("SHYFT_API_KEY must be set");

    // Sample transaction signature
    let tx_signature =
        "uZz2qDvLzndsTEY31YkxgRe1rYQ8MsCtS3DwhPYP1Md7u7dnUK4HW3vYzsxE6GSxFhSG5zpvqSQnSUn1sPmzTBu";

    // Create ShyftApi instance
    let shyft_api = ShyftApi::new(&api_key, None, None, None).expect("Failed to create ShyftApi");

    // Fetch parsed transaction details
    let parsed_transaction_details = shyft_api.get_transaction_parsed(tx_signature).await;

    // Assert that the parsed transaction details are fetched successfully
    assert!(
        parsed_transaction_details.is_ok(),
        "Failed to fetch parsed transaction details: {:?}",
        parsed_transaction_details.err()
    );

    // Assert that the protocol name in the parsed transaction details is as expected
    assert_eq!(
        parsed_transaction_details.unwrap().protocol.name,
        "JUPITER_V_6"
    );
}
