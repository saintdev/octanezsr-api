use async_trait::async_trait;
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use http::{header, Request};
use serde::{de::DeserializeOwned, Serialize};

use super::{
    common::Collection,
    endpoint::Endpoint,
    query::{AsyncQuery, Query},
    utils::{deserialize_response, url_to_http_uri},
    ApiError, AsyncClient, Client, RestClient,
};

/// Marker trait to indicate that an endpoint is pageable
pub trait Pageable {}

/// Adapters specific to [`Pageable`] endpoints
pub trait PagedEndpointExt<'a, E> {
    /// Create an Iterator over the results of the paginated endpoint
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: Client,
        T: DeserializeOwned;

    /// `GET` a single page of results for the paginated endpoint
    fn page(&'a self) -> PageBuilder<'a, E>;

    /// Create an async Stream over the results of the paginated endpoint
    fn stream<T, C>(&'a self, client: &'a C) -> BoxStream<'a, Result<T, ApiError<C::Error>>>
    where
        T: DeserializeOwned + Send + 'static,
        C: AsyncClient + Sync,
        E: Send + Sync;
}

impl<'a, E> PagedEndpointExt<'a, E> for E
where
    E: Endpoint + Pageable,
{
    fn iter<T, C>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T>
    where
        C: Client,
        T: DeserializeOwned,
    {
        PagedIter::new(self, client)
    }

    fn page(&'a self) -> PageBuilder<'a, E> {
        PageBuilder::new(self)
    }

    fn stream<T, C>(&'a self, client: &'a C) -> BoxStream<'a, Result<T, ApiError<C::Error>>>
    where
        T: DeserializeOwned + Send + 'static,
        C: AsyncClient + Sync,
        E: Send + Sync,
    {
        //FIXME: Set this back to 1
        futures::stream::try_unfold(Some(1), move |state| async move {
            let Some(page) = state else {
                return Ok::<_, ApiError<C::Error>>(None);
            };

            let page = Page::builder(self).page(page).build();
            let page = page.query_async(client).await?;

            let pagination = page.pagination.expect("Missing pagination info!");

            if page.inner.is_empty() {
                Ok(None)
            } else {
                let next_state = if pagination.page_size < pagination.per_page {
                    None
                } else {
                    Some(pagination.page + 1)
                };
                Ok(Some((
                    futures::stream::iter(page.inner.into_iter().map(Ok)),
                    next_state,
                )))
            }
        })
        .try_flatten()
        .boxed()
    }
}

/// Iterator type for the [`iter`] method on the [`PagedEndpointExt`] trait
///
/// [`iter`]: PagedEndpointExt::iter
#[derive(Debug)]
pub struct PagedIter<'a, E, C, T> {
    client: &'a C,
    state: Page<'a, E>,
    last_page: bool,
    current_page: Vec<T>,
}

impl<'a, E, C, T> PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
{
    pub(crate) fn new(paged: &'a E, client: &'a C) -> Self {
        let state = Page::builder(paged).page(1).build();
        Self {
            client,
            state,
            last_page: false,
            current_page: Vec::new(),
        }
    }
}

impl<'a, E, C, T> Iterator for PagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
    C: Client,
    T: DeserializeOwned,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            if self.last_page {
                return None;
            }

            self.current_page = match self.state.query(self.client) {
                Ok(page) => {
                    let pagination = page.pagination.expect("Missing pagination info");

                    if pagination.per_page < pagination.page_size {
                        self.last_page = true;
                    }

                    self.state.page = pagination.page + 1;

                    page.inner
                }
                Err(err) => return Some(Err(err)),
            };

            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

/// Builder for the [`Page`] endpoint
#[derive(Debug)]
pub struct PageBuilder<'a, E> {
    inner: &'a E,
    page: Option<usize>,
    per_page: Option<usize>,
}

impl<'a, E> PageBuilder<'a, E>
where
    E: Pageable + Endpoint,
{
    /// Create a new [`PageBuilder`]
    pub fn new(paged: &'a E) -> Self {
        Self {
            inner: paged,
            page: None,
            per_page: None,
        }
    }

    /// Page number. Example: `1`
    pub fn page<T>(mut self, value: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.page = value.into();
        self
    }

    /// Results per page. Example: `20`
    pub fn per_page<T>(mut self, value: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.per_page = value.into();
        self
    }

    /// Returns a [`Page`] that can be queried for a set of elements.
    pub fn build(self) -> Page<'a, E>
    where
        E: Pageable,
    {
        Page {
            inner: self.inner,
            page: self.page.unwrap_or(1),
            per_page: self.per_page,
        }
    }
}

/// Represents a single page of results
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<'a, E> {
    #[serde(skip)]
    inner: &'a E,
    #[serde(skip_serializing_if = "is_first_page")]
    page: usize,
    per_page: Option<usize>,
}

impl<'a, E> Page<'a, E>
where
    E: Endpoint + Pageable,
{
    /// Create a builder for a [`Page`]
    pub fn builder(paged: &'a E) -> PageBuilder<'a, E> {
        PageBuilder::new(paged)
    }

    fn page_url<C: RestClient>(&self, client: &C) -> Result<url::Url, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.inner.endpoint())?;
        let mut params = self.inner.query_parameters()?;
        params.extend_from(&self)?;
        params.apply_to(&mut url);
        Ok(url)
    }
}

impl<'a, E, T, C> Query<Collection<T>, C> for Page<'a, E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Collection<T>, ApiError<<C>::Error>> {
        let url = self.page_url(client)?;
        let url = url_to_http_uri(&url);

        let body = self.inner.body()?;

        let request = Request::builder()
            .method(self.inner.method())
            .uri(url.clone());

        let (request, body) = if let Some((mime, data)) = body {
            let request = request.header(header::CONTENT_TYPE, mime);
            (request, data)
        } else {
            (request, Vec::new())
        };

        let response = client.rest(request, body)?;

        deserialize_response::<_, C>(response).map_err(|e| ApiError::from_http_response(e, url))
    }
}

#[async_trait]
impl<'a, T, C, E> AsyncQuery<Collection<T>, C> for Page<'a, E>
where
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
    E: Endpoint + Pageable + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Collection<T>, ApiError<C::Error>> {
        let url = self.page_url(client)?;
        let url = url_to_http_uri(&url);

        let body = self.inner.body()?;

        let request = Request::builder()
            .method(self.inner.method())
            .uri(url.clone());

        let (request, body) = if let Some((mime, data)) = body {
            let request = request.header(header::CONTENT_TYPE, mime);
            (request, data)
        } else {
            (request, Vec::new())
        };

        let response = client.rest_async(request, body).await?;

        deserialize_response::<_, C>(response).map_err(|e| ApiError::from_http_response(e, url))
    }
}

fn is_first_page(value: &usize) -> bool {
    *value <= 1
}
