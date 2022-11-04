//! # Matches
//!
//! Endpoints for matches
mod games;
mod list_matches;
mod match_mod;

pub use games::*;
pub use list_matches::*;
pub use match_mod::*;

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

/// Represents a match id
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct MatchId<'a>(Cow<'a, str>);

impl<'a> MatchId<'a> {
    /// Create a new [`MatchId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for MatchId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for MatchId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
