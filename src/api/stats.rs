//! # Stats
//!
//! Endpoints for stats
mod players;
mod teams;

pub use players::{events::*, opponents::*, teams::*, *};
pub use teams::{events::*, opponents::*, *};
