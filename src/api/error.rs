use std::error::Error;

use thiserror::Error;

use super::utils::ResponseError;

/// Errors that originate from API endpoints
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Error URL encoding
    #[error("failed to create urlencoded form data: {0}")]
    Body(#[from] BodyError),
    /// The client encountered an error
    #[error("client error: {0}")]
    Client(E),
    /// The URL failed to parse
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),
    /// Error in HTTP response
    #[error("Error in HTTP response for url {url}: {source}")]
    Response {
        /// Source of the error
        source: ResponseError,
        /// URL of the error
        url: http::Uri,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub(crate) fn client(source: E) -> Self {
        Self::Client(source)
    }

    pub(crate) fn from_http_response(source: ResponseError, url: http::Uri) -> Self {
        Self::Response { source, url }
    }
}

#[derive(Debug, Error)]
pub enum BodyError {
    /// Error serializing form data
    #[error("URL encoding error: {0}")]
    UrlEncoding(#[from] serde_urlencoded::ser::Error),
}
