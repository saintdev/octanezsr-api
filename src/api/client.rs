use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use std::error::Error;
use url::Url;

use super::error::ApiError;

/// A trait representing a client that can communicate with the Octane ZSR API
/// via REST
pub trait RestClient {
    /// The error type returned by this client
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for this endpoint
    ///
    /// This method adds the hostname for the API
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a synchronous client that can communicate with the
/// Octane ZSR API
pub trait Client: RestClient {
    /// Send a REST query
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing a asynchronous client that can communicate with the
/// Octane ZSR API
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
