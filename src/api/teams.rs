//! # Teams
//!
//! Endpoints for teams
mod list_active_teams;
mod list_teams;
mod team;

pub use list_active_teams::*;
pub use list_teams::*;
pub use team::*;

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

/// Represents a team ID
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct TeamId<'a>(Cow<'a, str>);

impl<'a> TeamId<'a> {
    /// Create a new [`TeamId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for TeamId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for TeamId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
