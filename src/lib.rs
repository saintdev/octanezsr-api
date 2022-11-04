#![warn(
    future_incompatible,
    unused,
    missing_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    nonstandard_style
)]
#![warn(clippy::all)]
#![allow(rustdoc::broken_intra_doc_links)]
//! This crate implements a warpper for the Octane.gg ZSR API
//!
//! Endpoints are available in the [api](src/api.rs) module.

pub mod api;
mod client;
pub mod error;
pub mod types;

pub use api::pagination::PagedEndpointExt;
pub use client::{OctaneZsrBuilder, OctaneZsrClient, OctaneZsrClientAsync};
