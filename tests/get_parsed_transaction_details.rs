mod common;

use shyft_rs_sdk::{models::parsed_transaction_details::ParsedTransactionDetails, ShyftApi};

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

    let expected_data_json = serde_json::json!(
        {
            "timestamp": "2024-08-26T10:38:46.000Z",
            "fee": 0.000005734,
            "fee_payer": "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna",
            "signers": [
              "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna"
            ],
            "signatures": [
              "uZz2qDvLzndsTEY31YkxgRe1rYQ8MsCtS3DwhPYP1Md7u7dnUK4HW3vYzsxE6GSxFhSG5zpvqSQnSUn1sPmzTBu"
            ],
            "protocol": {
              "address": "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
              "name": "JUPITER_V_6"
            },
            "type": "SWAP",
            "status": "Success",
            "token_balance_changes": [
              {
                "address": "7ZtGCPMVkF1bZC7Tz1u3aTB8GE386pm6DgC7RT7kAm3A",
                "decimals": 6,
                "change_amount": -877265344778813700i64,
                "post_balance": 0,
                "pre_balance": 877265344778813700i64,
                "mint": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk",
                "owner": "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna"
              },
              {
                "address": "3t4BGaZCYCUHLkmEzcdqxqmjvPCxsHwzdLyf7qJi1bwW",
                "decimals": 6,
                "change_amount": 877265344778813700i64,
                "post_balance": 888391363782343000i64,
                "pre_balance": 11126019003529296i64,
                "mint": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk",
                "owner": "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1"
              },
              {
                "address": "6dWWrueT5BVTMwCwGVEFmf8r1jrM6WAbown8cY1Qgw8a",
                "decimals": 9,
                "change_amount": -80249704804i64,
                "post_balance": 1020326963i64,
                "pre_balance": 81270031767i64,
                "mint": "So11111111111111111111111111111111111111112",
                "owner": "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1"
              }
            ],
            "actions": [
              {
                "info": {
                  "swapper": "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna",
                  "tokens_swapped": {
                    "in": {
                      "token_address": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk",
                      "name": "DUROV TOKEN",
                      "symbol": "DUROV",
                      "image_uri": "https://gateway.pinata.cloud/ipfs/QmTjEWqwoPx2QxZbPH7T87x37tD4qMbh43BdeTZGhLgbZM",
                      "amount": 877265344778.8137,
                      "amount_raw": 877265344778813700i64
                    },
                    "out": {
                      "token_address": "So11111111111111111111111111111111111111112",
                      "name": "Wrapped SOL",
                      "symbol": "SOL",
                      "image_uri": "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png",
                      "amount": 80.249704804,
                      "amount_raw": 80249704804i64
                    }
                  },
                  "swaps": [
                    {
                      "liquidity_pool_address": "BfXa9jNX6h1yjAsCgDgEdEGtTaTsQqw6dHvyRaqyRWe1",
                      "name": "DUROV-SOL",
                      "source": "Raydium",
                      "in": {
                        "token_address": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk",
                        "name": "DUROV TOKEN",
                        "symbol": "DUROV",
                        "image_uri": "https://gateway.pinata.cloud/ipfs/QmTjEWqwoPx2QxZbPH7T87x37tD4qMbh43BdeTZGhLgbZM",
                        "amount": 877265344778.8137,
                        "amount_raw": 877265344778813700i64
                      },
                      "out": {
                        "token_address": "So11111111111111111111111111111111111111112",
                        "name": "Wrapped SOL",
                        "symbol": "SOL",
                        "image_uri": "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png",
                        "amount": 80.249704804,
                        "amount_raw": 80249704804i64
                      }
                    }
                  ],
                  "slippage_in_percent": 5,
                  "quoted_out_amount": 80179718017i64,
                  "slippage_paid": -0.06998678700000482
                },
                "source_protocol": {
                  "address": "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
                  "name": "JUPITER_V_6"
                },
                "type": "SWAP",
                "ix_index": 7
              },
              {
                "info": {
                  "amount": 877265344778.8137,
                  "amount_raw": 877265344778813700i64,
                  "sender": "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna",
                  "receiver_associated_account": "3t4BGaZCYCUHLkmEzcdqxqmjvPCxsHwzdLyf7qJi1bwW",
                  "receiver": "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
                  "token_address": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk"
                },
                "source_protocol": {
                  "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                  "name": "Token Program"
                },
                "type": "TOKEN_TRANSFER",
                "parent_protocol": "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
                "ix_index": 9
              },
              {
                "info": {
                  "amount": 80.249704804,
                  "amount_raw": 80249704804i64,
                  "sender": "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
                  "receiver_associated_account": "CoQFjx8fMMTmRKq3MDy7XyacHsp2pgFfUBGNNkLExU4W",
                  "receiver": "8R5brRqNa1CDMtcQRaLPQfJeLBrtyqpjDPTSKbBvmsna",
                  "token_address": "So11111111111111111111111111111111111111112"
                },
                "source_protocol": {
                  "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                  "name": "Token Program"
                },
                "type": "TOKEN_TRANSFER",
                "parent_protocol": "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
                "ix_index": 10
              }
            ],
            "events": [
              {
                "data": {
                  "amm": "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                  "inputMint": "EWFka4M5XZrKzAq54eTXxN5K1H3u2es1HVqvr6hsqdSk",
                  "inputAmount": 877265344778813700i64,
                  "outputMint": "So11111111111111111111111111111111111111112",
                  "outputAmount": 80249704804i64
                },
                "name": "SwapEvent"
              }
            ]
          }
    );
    let expected_data: ParsedTransactionDetails =
        serde_json::from_value(expected_data_json).expect("Failed to parse expected data");

    // Assert that the parsed transaction details match the expected data
    assert_eq!(parsed_transaction_details.unwrap(), expected_data);
}
