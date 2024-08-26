/// Base URL for the Shyft API
pub const URL: &str = "https://api.shyft.to/sol/v1/";

/// Minimum retry interval in milliseconds
pub const MIN_RETRY_INTERVAL: u64 = 500;
/// Maximum retry interval in milliseconds
pub const MAX_RETRY_INTERVAL: u64 = 1000;
/// Maximum number of retries
pub const MAX_RETRIES: u32 = 3;
