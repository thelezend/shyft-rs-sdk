use std::time::Duration;

use reqwest_retry::{
    policies::{ExponentialBackoff, ExponentialBackoffBuilder},
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};

/// Creates a retry strategy using exponential backoff.
///
/// # Arguments
///
/// * `min_retry_interval` - Minimum retry interval in milliseconds.
/// * `max_retry_interval` - Maximum retry interval in milliseconds.
/// * `max_retries` - Maximum number of retries.
///
/// # Returns
///
/// * `RetryTransientMiddleware<ExponentialBackoff, Retry>` - A middleware configured with the retry policy and strategy.
pub fn get_retry_strategy(
    min_retry_interval: u64,
    max_retry_interval: u64,
    max_retries: u32,
) -> RetryTransientMiddleware<ExponentialBackoff, Retry> {
    let retry_policy = ExponentialBackoffBuilder::default()
        .retry_bounds(
            Duration::from_millis(min_retry_interval),
            Duration::from_millis(max_retry_interval),
        )
        .build_with_max_retries(max_retries);

    RetryTransientMiddleware::new_with_policy_and_strategy(retry_policy, Retry)
}

/// A struct implementing the `RetryableStrategy` trait for handling retry logic.
pub struct Retry;

impl RetryableStrategy for Retry {
    /// Handles the retry logic based on the response or error.
    ///
    /// # Arguments
    ///
    /// * `res` - A result containing either a `reqwest::Response` or a `reqwest_middleware::Error`.
    ///
    /// # Returns
    ///
    /// * `Option<Retryable>` - An option indicating whether the request should be retried.
    fn handle(
        &self,
        res: &Result<reqwest::Response, reqwest_middleware::Error>,
    ) -> Option<Retryable> {
        match res {
            Ok(success) => {
                if success.status() != 200 {
                    Some(Retryable::Transient)
                } else {
                    None
                }
            }
            Err(error) => reqwest_retry::default_on_request_failure(error),
        }
    }
}
