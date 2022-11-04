use bytes::Bytes;
use http::{header, request::Builder as RequestBuilder, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serializer};
use thiserror::Error;

use super::{common::Direction, endpoint::Endpoint, ApiError, RestClient};

pub(crate) fn url_to_http_uri(url: &url::Url) -> http::Uri {
    url.as_str()
        .parse::<http::Uri>()
        .expect("failed to parse url::Url as http::Uri")
}

pub(crate) fn build_request<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
where
    E: Endpoint,
    C: RestClient,
{
    let mut url = client.rest_endpoint(&endpoint.endpoint())?;
    endpoint.query_parameters()?.apply_to(&mut url);
    let http_req = Request::builder()
        .method(endpoint.method())
        .uri(url_to_http_uri(&url));

    let (http_req, body) = if let Some((mime, data)) = endpoint.body()? {
        let http_req = http_req.header(header::CONTENT_TYPE, mime);
        (http_req, data)
    } else {
        (http_req, Vec::new())
    };
    Ok((http_req, body))
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Parsing JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Deserializing value: {source}")]
    DataType {
        source: serde_json::Error,
        value: serde_json::Value,
        typ: &'static str,
    },
    #[error("HTTP error: {status}")]
    HttpStatus {
        value: serde_json::Value,
        status: StatusCode,
    },
}

pub(crate) fn deserialize_response<T, C>(rsp: Response<Bytes>) -> Result<T, ResponseError>
where
    T: DeserializeOwned,
    C: RestClient,
{
    let status = rsp.status();
    let json_value = serde_json::from_slice(rsp.body())?;
    if !status.is_success() {
        return Err(ResponseError::HttpStatus {
            value: json_value,
            status,
        });
    }

    serde_json::from_value(json_value.clone()).map_err(|err| ResponseError::DataType {
        source: err,
        value: json_value,
        typ: std::any::type_name::<T>(),
    })
}

pub(crate) fn serialize_as_colon_separated<S, T>(
    value: &Option<(T, Direction)>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<&'static str> + Copy,
{
    if let Some(value) = value {
        let out = format!("{}:{}", value.0.into(), value.1.as_str());
        serializer.serialize_str(&out)
    } else {
        serializer.serialize_none()
    }
}
