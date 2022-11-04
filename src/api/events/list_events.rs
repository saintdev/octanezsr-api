use std::borrow::Cow;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

use crate::api::{
    common::{Direction, Mode, Region, Tier},
    endpoint::Endpoint,
    error::BodyError,
    pagination::Pageable,
    query_params::QueryParams,
};

/// Field to sort events by
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum EventSorting {
    /// Sort by `name`
    Name,
    /// Sort by `tier`
    Tier,
    /// Sort by `region`
    Region,
    /// Sort by `mode`
    Mode,
    /// Sort by `group`
    Group,
}

// FIXME: Is there a way to use serde to handle this automatically?
impl EventSorting {
    fn as_str(&self) -> &'static str {
        match self {
            EventSorting::Name => "name",
            EventSorting::Tier => "tier",
            EventSorting::Region => "region",
            EventSorting::Mode => "mode",
            EventSorting::Group => "group",
        }
    }
}

impl From<EventSorting> for &'static str {
    fn from(value: EventSorting) -> Self {
        value.as_str()
    }
}

/// `GET` a list of all events
#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ListEvents<'a> {
    #[doc = "A portion of the even name. Example: `\"RLCS\"`"]
    name: Option<Cow<'a, str>>,
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
    #[doc = "Filter events on this date. Example: `\"2016-12-03\".parse()`"]
    date: Option<DateTime<Utc>>,
    #[builder(setter(custom))]
    #[serde(serialize_with = "crate::api::utils::serialize_as_colon_separated")]
    sort: Option<(EventSorting, Direction)>,
}

impl ListEvents<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> ListEventsBuilder<'a> {
        ListEventsBuilder::default()
    }
}

impl ListEventsBuilder<'_> {
    /// Sorting method for results. Takes a field to sort by and a sort
    /// direction.
    pub fn sort<VALUE: Into<EventSorting>, D: Into<Direction>>(
        &mut self,
        value: VALUE,
        direction: D,
    ) -> &mut Self {
        self.sort = Some(Some((value.into(), direction.into())));
        self
    }
}

impl Endpoint for ListEvents<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        "events".into()
    }

    fn query_parameters(&self) -> Result<QueryParams<'_>, BodyError> {
        QueryParams::with(self)
    }
}

impl Pageable for ListEvents<'_> {}
