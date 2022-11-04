use std::borrow::Cow;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::{BestOf, Direction, Mode, Region, Tier},
    endpoint::Endpoint,
    error::BodyError,
    events::EventId,
    pagination::Pageable,
    players::PlayerId,
    query_params::QueryParams,
    stages::StageId,
    teams::TeamId,
};

/// Field to sort matches by
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MatchesSorting {
    /// Sort by `event`
    Event,
    /// Sort by `stage`
    Stage,
    /// Sort by `qualifier`
    Qualifier,
    /// Sort by `tier`
    Tier,
    /// Sort by `region`
    Region,
    /// Sort by `mode`
    Mode,
    /// Sort by `group`
    Group,
    /// Sort by `before`
    Before, //TODO: Is this valid?
    /// Sort by `after`
    After, //TODO: Is this valid?
    /// Sort by `date`
    Date,
    /// Sort by `reverseSweep`
    ReverseSweep,
    /// Sort by `reverseSweepAttempt`
    ReverseSweepAttempt,
    /// Sort by `player`
    Player,
    /// Sort by `team`
    Team,
}

// FIXME: Is there a way to use serde to handle this automatically?
impl MatchesSorting {
    fn as_str(&self) -> &'static str {
        match self {
            MatchesSorting::Event => "event",
            MatchesSorting::Stage => "stage",
            MatchesSorting::Qualifier => "qualifier",
            MatchesSorting::Tier => "tier",
            MatchesSorting::Region => "region",
            MatchesSorting::Mode => "mode",
            MatchesSorting::Group => "group",
            MatchesSorting::Before => "before",
            MatchesSorting::After => "after",
            MatchesSorting::Date => "date",
            MatchesSorting::ReverseSweep => "reverseSweep",
            MatchesSorting::ReverseSweepAttempt => "referseSweepAttempt",
            MatchesSorting::Player => "player",
            MatchesSorting::Team => "team",
        }
    }
}

impl From<MatchesSorting> for &'static str {
    fn from(value: MatchesSorting) -> Self {
        value.as_str()
    }
}

/// `GET` a list of all matches
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(default, setter(into, strip_option))]
pub struct ListMatches<'a> {
    #[doc = "An event id. Example: `\"5f35882d53fbbb5894b43040\"`"]
    event: Option<EventId<'a>>,
    #[doc = "A stage id. Example: `1`"]
    stage: Option<StageId>,
    #[doc = "Is stage a qualifier. Example: `true`"]
    qualifier: Option<bool>,
    #[doc = "An event tier. Example: `Tier::S`"]
    tier: Option<Tier>,
    #[doc = "An event region. Example: `Region::NA`"]
    region: Option<Region>,
    #[doc = "An event mode. Example: `Mode::Three`"]
    mode: Option<Mode>,
    #[doc = "An event group. Example: `\"rlcsx\"`"]
    group: Option<Cow<'a, str>>,
    #[doc = "Filter events before this date. Example: `\"2016-12-03\".parse()`"]
    before: Option<DateTime<Utc>>,
    #[doc = "Filter events after this date. Example: `\"2016-12-03\".parse()`"]
    after: Option<DateTime<Utc>>,
    #[doc = "A match format. Example: `BestOf::Five`"]
    best_of: Option<BestOf>,
    #[doc = "Is match a reverse sweep. Example: `true`"]
    reverse_sweep: Option<bool>,
    #[doc = "Is match a reverse sweep attempt. Example: `true`"]
    reverse_sweep_attempt: Option<bool>,
    #[doc = "A player id. Example: `\"5f3d8fdd95f40596eae23d97\"`"]
    player: Option<PlayerId<'a>>,
    #[doc = "A team id. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    team: Option<TeamId<'a>>,
    #[builder(setter(custom))]
    #[serde(serialize_with = "crate::api::utils::serialize_as_colon_separated")]
    sort: Option<(MatchesSorting, Direction)>,
}

impl ListMatches<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> ListMatchesBuilder<'a> {
        ListMatchesBuilder::default()
    }
}

impl ListMatchesBuilder<'_> {
    /// Sorting method for results. Takes a field to sort by and a sort
    /// direction.
    pub fn sort<VALUE: Into<MatchesSorting>, D: Into<Direction>>(
        &mut self,
        value: VALUE,
        direction: D,
    ) -> &mut Self {
        self.sort = Some(Some((value.into(), direction.into())));
        self
    }
}

impl Endpoint for ListMatches<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/matches".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(&self)
    }
}

impl Pageable for ListMatches<'_> {}
