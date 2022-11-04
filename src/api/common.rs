use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The root type returned by queries that return a collection endpoint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection<T> {
    /// The inner collection
    #[serde(alias = "events")]
    #[serde(alias = "matches")]
    #[serde(alias = "games")]
    #[serde(alias = "players")]
    #[serde(alias = "teams")]
    #[serde(alias = "records")]
    #[serde(alias = "participants")]
    pub inner: Vec<T>,
    /// Pagination metadata, if present
    #[serde(flatten)]
    pub pagination: Option<Pagination>,
}

impl<T> Collection<T> {
    /// Consumes the [`Collection`] returning the wrapped value.
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}

/// Pagination metadata
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Current page in the result set
    pub page: usize,
    /// Requested number of records per page
    pub per_page: usize,
    /// Number of records returned in this page
    pub page_size: usize,
}

/// Event tier
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    Monthly,
    Weekly,
    #[serde(rename = "Show Match")]
    ShowMatch,
    Qualifier,
}

/// Event region
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    #[serde(rename = "NA")]
    NorthAmerica,
    #[serde(rename = "EU")]
    Europe,
    #[serde(rename = "OCE")]
    Oceania,
    #[serde(rename = "SAM")]
    SouthAmerica,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "ME")]
    MiddleEast,
    #[serde(rename = "INT")]
    International,
    #[serde(rename = "AF")]
    Africa,
}

/// Event mode
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Mode {
    One = 1,
    Two = 2,
    Three = 3,
}

/// Match format
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BestOf {
    Three = 3,
    Five = 5,
    Seven = 7,
}

/// Sort direction
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

// FIXME: Is there a way to use serde to handle this automatically?
impl Direction {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }
    }
}
