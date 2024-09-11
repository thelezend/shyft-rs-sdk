mod common;

use shyft_rs_sdk::ShyftApi;

#[tokio::main]
async fn main() {
    common::setup();

    // Retrieve the API key from environment variables
    let api_key = std::env::var("SHYFT_API_KEY").expect("SHYFT_API_KEY must be set");

    // Sample transaction signature
    let tx_signature =
        "uZz2qDvLzndsTEY31YkxgRe1rYQ8MsCtS3DwhPYP1Md7u7dnUK4HW3vYzsxE6GSxFhSG5zpvqSQnSUn1sPmzTBu";

    // Create ShyftApi instance
    let shyft_api =
        ShyftApi::new(&api_key, None, None, None, None, None).expect("Failed to create ShyftApi");

    // Fetch parsed transaction details
    let parsed_transaction_details = shyft_api.get_transaction_parsed(tx_signature).await;

    println!("{:?}", parsed_transaction_details);
}
