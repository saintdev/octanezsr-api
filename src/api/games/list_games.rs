use std::borrow::Cow;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::{BestOf, Direction, Mode, Region, Tier},
    endpoint::Endpoint,
    error::BodyError,
    events::EventId,
    matches::MatchId,
    pagination::Pageable,
    players::PlayerId,
    query_params::QueryParams,
    stages::StageId,
    teams::TeamId,
};

/// Field to sort games by
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum GamesSorting {
    /// Sort by `event`
    Event,
    /// Sort by `stage`
    Stage,
    /// Sort by `match`
    Match,
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
    Before, //FIXME: Is this valid?
    /// Sort by `after`
    After, //FIXME: Is this valid?
    /// Sort by `bestOf`
    BestOf,
    /// Sort by `player`
    Player,
    /// Sort by `team`
    Team,
    /// Sort by `date`
    Date,
}

impl GamesSorting {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            GamesSorting::Event => "event",
            GamesSorting::Stage => "stage",
            GamesSorting::Match => "match",
            GamesSorting::Qualifier => "qualifier",
            GamesSorting::Tier => "tier",
            GamesSorting::Region => "region",
            GamesSorting::Mode => "mode",
            GamesSorting::Group => "group",
            GamesSorting::Before => "before",
            GamesSorting::After => "after",
            GamesSorting::BestOf => "bestOf",
            GamesSorting::Player => "player",
            GamesSorting::Team => "team",
            GamesSorting::Date => "date",
        }
    }
}

// FIXME: Is there a way to use serde to handle this automatically?
impl From<GamesSorting> for &'static str {
    fn from(value: GamesSorting) -> Self {
        value.as_str()
    }
}

/// `GET` a list of all games
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(default, setter(into, strip_option))]
pub struct ListGames<'a> {
    #[doc = "An event id. Example: `\"5f35882d53fbbb5894b43040\"`"]
    event: Option<EventId<'a>>,
    #[doc = "A stage id. Example: `1`"]
    stage: Option<StageId>,
    #[doc = "A match id. Example: `\"6043152fa09e7fba40d2ae62\"`"]
    #[serde(rename = "match")]
    match_id: Option<MatchId<'a>>,
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
    #[doc = "Filter matches before this date. Example: `\"2016-12-03\".parse()`"]
    before: Option<DateTime<Utc>>,
    #[doc = "Filter matches after this date. Example: `\"2016-12-03\".parse()`"]
    after: Option<DateTime<Utc>>,
    #[doc = "A match format. Example: `BestOf::Five`"]
    best_of: Option<BestOf>,
    #[doc = "A player id. Example: `\"5f3d8fdd95f40596eae23d97\"`"]
    player: Option<PlayerId<'a>>,
    #[doc = "A team id. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    team: Option<TeamId<'a>>,
    #[builder(setter(custom))]
    #[serde(serialize_with = "crate::api::utils::serialize_as_colon_separated")]
    sort: Option<(GamesSorting, Direction)>,
}

impl ListGames<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> ListGamesBuilder<'a> {
        ListGamesBuilder::default()
    }
}

impl ListGamesBuilder<'_> {
    /// Sorting method for results. Takes a field to sort by and a sort
    /// direction.
    pub fn sort<VALUE: Into<GamesSorting>, D: Into<Direction>>(
        &mut self,
        value: VALUE,
        direction: D,
    ) -> &mut Self {
        self.sort = Some(Some((value.into(), direction.into())));
        self
    }
}

impl Endpoint for ListGames<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "/games".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for ListGames<'_> {}
