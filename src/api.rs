//! API endpoint modules
//!
//! All endpoints use a builder pattern to construct their parameters.
//!
//! # Example
//!
//! ```rust ,no_run
//! use futures::{StreamExt, TryStreamExt};
//! use octanezsr_api::{
//!     api::{self, AsyncQuery},
//!     error::OctaneZsrResult,
//!     OctaneZsrBuilder, PagedEndpointExt,
//! };
//! use serde::Deserialize;
//!
//! #[derive(Debug, Deserialize)]
//! struct Event<'a> {
//!     #[serde(rename = "_id")]
//!     id: api::events::EventId<'a>,
//!     name: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct GameScore {
//!     blue: i64,
//!     orange: i64,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Match<'a> {
//!     #[serde(rename = "_id")]
//!     id: api::matches::MatchId<'a>,
//!     #[serde(default)]
//!     pub games: Vec<GameScore>,
//! }
//!
//! #[tokio::main]
//! pub async fn main() -> OctaneZsrResult<()> {
//!     // Create a new async client.
//!     let client = OctaneZsrBuilder::new().build_async()?;
//!
//!     // Create an endpoint. This endpoint returns the RLCS Season 2 World
//!     // Championship
//!     let endpoint = api::events::Event::builder()
//!         .id("5f35882d53fbbb5894b43040")
//!         .build()
//!         .unwrap();
//!     // Call the endpoint. The return type decides how to represent the returned
//!     // value.
//!     let event: Event = endpoint.query_async(&client).await?;
//!
//!     println!("{}", event.name);
//!
//!     // Create a paginated endpoint. This retrieves a list of all matches in the RLCS
//!     // Season 2 World Champonship
//!     let endpoint = api::matches::ListMatches::builder()
//!         .event(event.id)
//!         .build()
//!         .unwrap();
//!     // The `PagedEndpointExt` adapters consume the endpoint, so create a copy.
//!     let async_stream = endpoint.clone();
//!     // Call the `PagedEndpointExt::stream()` method to get an async Stream of
//!     // results.
//!     let matches: Vec<Match> = async_stream.stream(&client).take(2).try_collect().await?;
//!
//!     for match_value in matches {
//!         println!("{:#?}", match_value);
//!     }
//!
//!     // Call the `PagedEndpointExt::page()` method to get a builder for a single page
//!     // of results
//!     let single_page_endpoint = endpoint.page().per_page(5).page(3).build();
//!     // The wrapped endpoint can be queried like a normal endpoint, but always
//!     // returns a `api::Root<T: Deserialize>`
//!     let matches_page: api::Root<Match> = single_page_endpoint.query_async(&client).await?;
//!
//!     println!("{:#?}", matches_page.inner[0]);
//!
//!     Ok(())
//! }
//! ```
mod client;
pub(crate) mod common;
mod endpoint;
mod error;
pub(crate) mod pagination;
mod query;
mod query_params;
pub(crate) mod utils;

pub mod events;
pub mod games;
pub mod matches;
pub mod players;
pub mod records;
pub mod stages;
pub mod stats;
pub mod teams;

pub use client::{AsyncClient, Client, RestClient};
pub use common::{Collection, Direction};
pub use error::ApiError;
pub use pagination::{Page, PageBuilder, Pageable, PagedEndpointExt, PagedIter};
pub use query::{AsyncQuery, Query};
pub(crate) use query_params::QueryParams;
