use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, response::Builder as ResponseBuilder, Response};
use log::debug;
use reqwest::{blocking::Client as HttpClient, Client as AsyncHttpClient};
use url::Url;

use crate::{
    api,
    error::{OctaneZsrResult, RestError},
};

const ZSR_API_BASE_URL: &str = "https://zsr.octane.gg/";

/// A client for communicating with the Octane ZSR API
#[derive(Clone, Debug)]
pub struct OctaneZsrClient {
    client: HttpClient,
    rest_url: Url,
}

impl OctaneZsrClient {
    /// Create a new Octane ZSR client.
    pub fn new() -> OctaneZsrResult<Self> {
        let rest_url = Url::parse(ZSR_API_BASE_URL)?;

        Ok(Self {
            client: HttpClient::new(),
            rest_url,
        })
    }

    /// Create a new Octane ZSR API builder.
    pub fn builder() -> OctaneZsrBuilder {
        OctaneZsrBuilder::new()
    }
}

impl api::RestClient for OctaneZsrClient {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        Ok(self.rest_url.join(endpoint.trim_start_matches('/'))?)
    }
}

impl api::Client for OctaneZsrClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            let request: reqwest::blocking::Request = request.body(body)?.try_into()?;

            debug!("{}", &request.url());

            let rsp = self.client.execute(request)?;

            let http_rsp = build_http_response(rsp.status(), rsp.version(), rsp.headers());

            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(api::ApiError::client)
    }
}

/// An asynchronous client for communicating with the Octane ZSR API
pub struct OctaneZsrClientAsync {
    client: AsyncHttpClient,
    rest_url: Url,
}

impl OctaneZsrClientAsync {
    /// Create a new asynchronous Octane ZSR client
    pub fn new() -> OctaneZsrResult<Self> {
        let rest_url = Url::parse(ZSR_API_BASE_URL)?;

        Ok(Self {
            client: AsyncHttpClient::new(),
            rest_url,
        })
    }

    /// Create a new Octane ZSR API builder.
    pub fn builder() -> OctaneZsrBuilder {
        OctaneZsrBuilder::new()
    }
}

impl api::RestClient for OctaneZsrClientAsync {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        Ok(self.rest_url.join(endpoint.trim_start_matches('/'))?)
    }
}

#[async_trait]
impl api::AsyncClient for OctaneZsrClientAsync {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, api::ApiError<Self::Error>> {
        let call = || async {
            let request: reqwest::Request = request.body(body)?.try_into()?;

            debug!("{}", &request.url());

            let rsp = self.client.execute(request).await?;

            let http_rsp = build_http_response(rsp.status(), rsp.version(), rsp.headers());

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().await.map_err(api::ApiError::client)
    }
}

/// Octane ZSR API client builder
#[derive(Debug, Default)]
pub struct OctaneZsrBuilder;

impl OctaneZsrBuilder {
    /// Create a new Octane ZSR API builder.
    pub fn new() -> Self {
        OctaneZsrBuilder::default()
    }

    /// Build a blocking Octane ZSR client.
    pub fn build(&self) -> OctaneZsrResult<OctaneZsrClient> {
        OctaneZsrClient::new()
    }

    /// Build an asynchronous Octane ZSR client.
    pub fn build_async(&self) -> OctaneZsrResult<OctaneZsrClientAsync> {
        OctaneZsrClientAsync::new()
    }
}

fn build_http_response(
    status: http::StatusCode,
    version: http::Version,
    headers: &http::HeaderMap,
) -> ResponseBuilder {
    let mut builder = http::Response::builder().status(status).version(version);
    if let Some(rsp_headers) = builder.headers_mut() {
        for (key, value) in headers {
            rsp_headers.insert(key, value.clone());
        }
    }
    builder
}
