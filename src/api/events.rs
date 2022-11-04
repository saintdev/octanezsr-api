//! # Events
//!
//! Endpoints for events
mod event;
mod list_events;
mod matches;
mod participants;

pub use event::*;
pub use list_events::*;
pub use matches::*;
pub use participants::*;

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

/// Represents an event id
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct EventId<'a>(Cow<'a, str>);

impl<'a> EventId<'a> {
    /// Create a new [`EventId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(id.into())
    }
}

impl<'a, T> From<T> for EventId<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for EventId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
