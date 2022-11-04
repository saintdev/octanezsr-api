//! Error types for this crate
use thiserror::Error;

use crate::api;

/// An alias for result types returned by this crate
pub type OctaneZsrResult<T> = Result<T, OctaneZsrError>;

/// Errors from the Octane ZSR API client
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OctaneZsrError {
    /// Error from the Octane ZSR API
    #[error("API error: {0}")]
    Api(#[from] api::ApiError<RestError>),
    /// Error parsing URL
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),
}

/// Errors communicating with the REST endpoint.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    /// Reqwest client error
    #[error("Communication error: {0}")]
    Communication(#[from] reqwest::Error),
    /// HTTP protocol error
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
}
