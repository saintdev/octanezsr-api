use std::borrow::Cow;

use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::Direction, endpoint::Endpoint, error::BodyError, pagination::Pageable,
    query_params::QueryParams, teams::TeamId,
};

/// Field to sort players by
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayersSorting {
    /// Sort by `tag`
    Tag,
    /// Sort by `country`
    Country,
    /// Sort by `team`
    Team,
}

impl PlayersSorting {
    fn as_str(&self) -> &'static str {
        match self {
            PlayersSorting::Tag => "tag",
            PlayersSorting::Country => "country",
            PlayersSorting::Team => "team",
        }
    }
}

impl From<PlayersSorting> for &'static str {
    fn from(value: PlayersSorting) -> Self {
        value.as_str()
    }
}

/// `GET` a list of all players
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(default, setter(into, strip_option))]
pub struct ListPlayers<'a> {
    #[doc = "A portion of the player tag. Example: `\"Kro\"`"]
    tag: Option<Cow<'a, str>>,
    #[doc = "A 2-letter country code. Example: `\"us\"`"]
    country: Option<Cow<'a, str>>,
    #[doc = "A team ID. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    team: Option<TeamId<'a>>,
    #[builder(setter(custom))]
    #[serde(serialize_with = "crate::api::utils::serialize_as_colon_separated")]
    sort: Option<(PlayersSorting, Direction)>,
}

impl ListPlayers<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> ListPlayersBuilder<'a> {
        ListPlayersBuilder::default()
    }
}

impl ListPlayersBuilder<'_> {
    /// Sorting method for results. Takes a field to sort by and a sort
    /// direction.
    pub fn sort<VALUE: Into<PlayersSorting>, D: Into<Direction>>(
        &mut self,
        value: VALUE,
        direction: D,
    ) -> &mut Self {
        self.sort = Some(Some((value.into(), direction.into())));
        self
    }
}

impl Endpoint for ListPlayers<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/players".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for ListPlayers<'_> {}
