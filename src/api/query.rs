use async_trait::async_trait;

use super::{ApiError, AsyncClient, Client};

/// Query made to a client
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform a query against the client
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// Asynchronous query made to a client
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform an asynchronous query against the client
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
