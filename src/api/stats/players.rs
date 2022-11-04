pub(crate) mod events;
pub(crate) mod opponents;
pub(crate) mod teams;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::{BestOf, Mode, Region, Tier},
    endpoint::Endpoint,
    error::BodyError,
    events::EventId,
    matches::MatchId,
    players::PlayerId,
    query_params::QueryParams,
    stages::StageId,
    teams::TeamId,
};

/// `GET` player stats
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct PlayerStats<'a> {
    #[doc = "Stat names. Example: `\"score\"`"]
    stat: Cow<'a, str>,
    #[doc = "An event ID. Example: `\"5f35882d53fbbb5894b43040\"`"]
    event: Option<EventId<'a>>,
    #[doc = "A stage ID. Example: `1`"]
    stage: Option<StageId>,
    #[doc = "A match ID. Example: `\"6043152fa09e7fba40d2ae62\"`"]
    #[serde(rename = "match")]
    match_id: Option<MatchId<'a>>,
    #[doc = "Is stage a qualifier. Example: `true`"]
    qualifier: Option<bool>,
    #[doc = "Game or series winner. Example: `true`"]
    winner: Option<bool>,
    #[doc = "A 2-letter country code. Example: `\"us\"`"]
    nationality: Option<Cow<'a, str>>,
    #[doc = "Event tier. Example `Tier::S`"]
    tier: Option<Tier>,
    #[doc = "Event region. Example: `Region::NA`"]
    region: Option<Region>,
    #[doc = "Event mode. Example: `Mode::Three`"]
    mode: Option<Mode>,
    #[doc = "Event group. Example: `\"rlcsx\"`"]
    group: Option<Cow<'a, str>>,
    #[doc = "Filter events before this date. Example: `\"2016-12-03\".parse()`"]
    before: Option<DateTime<Utc>>,
    #[doc = "Filter events after this date. Example: `\"2016-12-03\".parse()`"]
    after: Option<DateTime<Utc>>,
    #[doc = "A match format. Example: `BestOf::Five`"]
    best_of: Option<BestOf>,
    #[doc = "A player ID. Example: `\"5f3d8fdd95f40596eae23d97\"`"]
    player: Option<PlayerId<'a>>,
    #[doc = "A team ID. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    team: Option<TeamId<'a>>,
}

impl PlayerStats<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> PlayerStatsBuilder<'a> {
        PlayerStatsBuilder::default()
    }
}

impl Endpoint for PlayerStats<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/stats/players".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}
