pub(super) mod events;
pub(super) mod opponents;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::{BestOf, Mode, Region, Tier},
    endpoint::Endpoint,
    events::EventId,
    matches::MatchId,
    stages::StageId,
    teams::TeamId,
};

/// `GET` team stats
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct TeamStats<'a> {
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
    #[doc = "A team ID. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    team: Option<TeamId<'a>>,
}

impl TeamStats<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> TeamStatsBuilder<'a> {
        TeamStatsBuilder::default()
    }
}

impl Endpoint for TeamStats<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/stats/teams".into()
    }
}
