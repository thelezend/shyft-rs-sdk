/*!
This module contains the data structures for parsed transaction details.

<div class="warning">
This module does not necessarily cover all transaction types.
</div>
*/

use serde::{Deserialize, Serialize};

/// Struct representing parsed transaction details.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ParsedTransactionDetails {
    /// Timestamp of the transaction.
    pub timestamp: String,
    /// Fee associated with the transaction.
    pub fee: f64,
    /// Fee payer of the transaction.
    pub fee_payer: String,
    /// List of signers of the transaction.
    pub signers: Vec<String>,
    /// List of signatures of the transaction.
    pub signatures: Vec<String>,
    /// Protocol information.
    pub protocol: Protocol,
    /// Type of the transaction.
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Status of the transaction.
    pub status: String,
    /// List of actions in the transaction.
    pub actions: Vec<Action>,
    /// Raw transaction details.
    pub raw: Option<serde_json::Value>,
    /// Optional list of events in the transaction.
    pub events: Option<Vec<serde_json::Value>>,
}

/// Struct representing protocol information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Protocol {
    /// Address of the protocol.
    pub address: String,
    /// Name of the protocol.
    pub name: String,
}

/// Struct representing an action in the transaction.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Action {
    /// Information about the action.
    pub info: serde_json::Value,
    /// Source protocol of the action.
    pub source_protocol: Protocol,
    /// Type of the action.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Optional parent protocol of the action.
    pub parent_protocol: Option<String>,
    /// Optional index of the instruction.
    pub ix_index: Option<u32>,
}

/// Struct representing an event in the transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {}

/// Enum representing different types of information in the transaction.
#[derive(Debug, Serialize, Deserialize)]
pub enum Info {
    /// Information about pool creation.
    CreatePool(CreatePool),
    /// Information about SOL transfer.
    SolTransfer(SolTransfer),
    /// Information about token creation.
    TokenCreate(TokenCreate),
    /// Information about token minting.
    TokenMint(TokenMint),
    /// Information about token transfer.
    TokenTransfer(TokenTransfer),
    /// Information about PumpfunSwap.
    PumpfunSwap(Swap),
}

/// Struct representing pool creation information.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePool {
    /// Address of the pool creator.
    pub pool_creator: String,
    /// Address of the liquidity pool.
    pub liquidity_pool_address: String,
    /// Address of the first token mint.
    pub token_mint_one: String,
    /// Address of the second token mint.
    pub token_mint_two: String,
    /// Address of the first token vault.
    pub token_vault_one: String,
    /// Address of the second token vault.
    pub token_vault_two: String,
}

/// Struct representing token creation information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCreate {
    /// Address of the created token.
    pub token_address: String,
}

/// Struct representing token minting information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMint {
    /// Address of the minted token.
    pub token_address: String,
    /// Amount of tokens minted.
    pub amount: u64,
    /// Raw amount of tokens minted.
    pub amount_raw: u64,
    /// Address of the receiver of the minted tokens.
    pub receiver_address: String,
}

/// Struct representing token transfer information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenTransfer {
    /// Amount of tokens transferred.
    pub amount: u64,
    /// Raw amount of tokens transferred.
    pub amount_raw: u64,
    /// Address of the receiver.
    pub receiver: String,
    /// Address of the sender.
    pub sender: String,
    /// Associated account of the receiver.
    pub receiver_associated_account: String,
    /// Address of the transferred token.
    pub token_address: String,
}

/// Struct representing SOL transfer information.
#[derive(Debug, Serialize, Deserialize)]
pub struct SolTransfer {
    /// Address of the sender.
    pub sender: String,
    /// Address of the receiver.
    pub receiver: String,
    /// Amount of SOL transferred.
    pub amount: f64,
    /// Raw amount of SOL transferred.
    pub amount_raw: f64,
}

/// Struct representing Swap information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Swap {
    /// Address of the swapper.
    pub swapper: String,
    /// Tokens swapped information.
    pub tokens_swapped: TokensSwapped,
    /// List of swaps.
    pub swaps: Vec<serde_json::Value>,
    /// Slippage in percent.
    pub slippage_in_percent: Option<f64>,
    /// Quoted out amount.
    pub quoted_out_amount: Option<f64>,
    /// Slippage paid.
    pub slippage_paid: Option<f64>,
}

/// Struct representing tokens swapped.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokensSwapped {
    /// Token swapped in.
    #[serde(rename = "in")]
    pub token_in: TokenInfo,
    /// Token swapped out.
    #[serde(rename = "out")]
    pub token_out: TokenInfo,
}

/// Struct representing token information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Address of the token.
    pub token_address: String,
    /// Name of the token.
    pub name: String,
    /// Symbol of the token.
    pub symbol: String,
    /// Image URI of the token.
    pub image_uri: String,
    /// Amount of the token.
    pub amount: f64,
    /// Raw amount of the token.
    pub amount_raw: u64,
}
