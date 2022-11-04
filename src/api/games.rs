//! # Games
//!
//! Endpoints for games
mod game;
mod list_games;

pub use game::*;
pub use list_games::*;

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

/// Represents a game id
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct GameId<'a>(Cow<'a, str>);

impl<'a> GameId<'a> {
    /// Create a new [`GameId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for GameId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for GameId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
