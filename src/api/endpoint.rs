use std::borrow::Cow;

use async_trait::async_trait;

use http::Method;
use serde::de::DeserializeOwned;

use super::{
    error::BodyError,
    query::{AsyncQuery, Query},
    query_params::QueryParams,
    utils::{build_request, deserialize_response},
    ApiError, AsyncClient, Client,
};

pub trait Endpoint {
    fn endpoint(&self) -> Cow<'static, str>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        Ok(QueryParams::default())
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let (http_req, body) = build_request(self, client)?;
        let url = http_req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest(http_req, body)?;

        deserialize_response::<_, C>(rsp).map_err(|e| ApiError::from_http_response(e, url))
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let (http_req, body) = build_request(self, client)?;
        let url = http_req.uri_ref().cloned().unwrap_or_default();

        let rsp = client.rest_async(http_req, body).await?;

        deserialize_response::<_, C>(rsp).map_err(|e| ApiError::from_http_response(e, url))
    }
}
