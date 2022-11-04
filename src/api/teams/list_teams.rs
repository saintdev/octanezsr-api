use std::borrow::Cow;

use derive_builder::Builder;

use serde::Serialize;

use crate::api::{
    common::Direction, endpoint::Endpoint, error::BodyError, pagination::Pageable,
    query_params::QueryParams,
};

/// Field to sort teams by
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TeamsSorting {
    /// Sort by `name`
    Name,
}

// FIXME: Is there a way to use serde to handle this automatically?
impl TeamsSorting {
    fn as_str(&self) -> &'static str {
        match self {
            TeamsSorting::Name => "name",
        }
    }
}

impl From<TeamsSorting> for &'static str {
    fn from(value: TeamsSorting) -> Self {
        value.as_str()
    }
}

/// `GET` a list of all teams
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(default, setter(into, strip_option))]
pub struct ListTeams<'a> {
    #[doc = "A portion of the team name. Example: `\"Flip\"`"]
    name: Option<Cow<'a, str>>,
    #[builder(setter(custom))]
    #[serde(serialize_with = "crate::api::utils::serialize_as_colon_separated")]
    sort: Option<(TeamsSorting, Direction)>,
}

impl ListTeams<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> ListTeamsBuilder<'a> {
        ListTeamsBuilder::default()
    }
}

impl ListTeamsBuilder<'_> {
    /// Sorting method for results. Takes a field to sort by and a sort
    /// direction.
    pub fn sort<VALUE: Into<TeamsSorting>, D: Into<Direction>>(
        &mut self,
        value: VALUE,
        direction: D,
    ) -> &mut Self {
        self.sort = Some(Some((value.into(), direction.into())));
        self
    }
}

impl Endpoint for ListTeams<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/teams".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for ListTeams<'_> {}
