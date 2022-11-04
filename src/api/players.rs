//! # Players
//!
//! Endpoints for players
mod list_players;
mod player;

pub use list_players::*;
pub use player::*;

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

/// Represents a player ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct PlayerId<'a>(Cow<'a, str>);

impl<'a> PlayerId<'a> {
    /// Create a new [`PlayerId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for PlayerId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for PlayerId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
