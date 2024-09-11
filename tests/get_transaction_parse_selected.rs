mod common;

use shyft_rs_sdk::ShyftApi;

#[tokio::test]
async fn test_get_transaction_parse_selected() {
    common::setup();

    // Retrieve the API key from environment variables
    let api_key = std::env::var("SHYFT_API_KEY").expect("SHYFT_API_KEY must be set");

    // Sample transaction signatures
    let transaction_signatures = [
        "2WjUUuMHciuwLXPgSmddyHF9qPfZ15wcja9G1hprSKfk3JLhrhsyGL2VjVfA8apEMmpskvfwBX34nJ3btihGn2rH"
            .to_owned(),
        "3uYZVwazovut7WrgLC7VXbhfcysX1VDo8ugDfx2SJ4YHBZMkmieh4A7b2hPZghqDJr15ZUeCpVvBUEr8FE3nzz4U"
            .to_owned(),
    ];

    // Create ShyftApi instance
    let shyft_api =
        ShyftApi::new(&api_key, None, None, None, None, None).expect("Failed to create ShyftApi");

    // Fetch parsed transaction details for the given signatures
    let parsed_transaction_details = shyft_api
        .get_transaction_parse_selected(&transaction_signatures, None, None)
        .await;

    // Assert that the parsed transaction details are fetched successfully
    assert!(
        parsed_transaction_details.is_ok(),
        "Failed to fetch parsed transaction details: {:?}",
        parsed_transaction_details.err()
    );
    // Assert that the parsed transaction details are not empty
    assert!(
        !parsed_transaction_details.unwrap().is_empty(),
        "Parsed transaction details are empty"
    );
}
