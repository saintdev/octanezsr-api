//! # Records
//! 
//! Endpoints for records
mod games;
mod players;
mod series;
mod teams;

pub use games::*;
pub use players::*;
pub use series::*;
pub use teams::*;

use serde::Serialize;

/// Aggegration type for records
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AggregationType {
    /// Aggegrate over a single game
    Game,
    /// Aggegrate over a series
    Series,
}
