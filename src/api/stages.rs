//! # Stages
//!
//! Types for stages

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents a stage ID
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StageId(u32);

impl StageId {
    /// Create a new [`StageId`]
    pub fn new<T>(id: T) -> Self
    where
        T: Into<u32>,
    {
        Self(id.into())
    }
}

impl<T> From<T> for StageId
where
    T: Into<u32>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}

impl Display for StageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
