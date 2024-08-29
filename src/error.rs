//! This module defines custom error types for the Shyft API.

/// Enum representing custom error types for the Shyft API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error originating from the reqwest library.
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Error originating from the reqwest middleware.
    #[error("Reqwest middleware error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    /// Error indicating that the response status was not 200 OK.
    #[error("Response status not 200: {0}")]
    StatusNot200(String),
}
