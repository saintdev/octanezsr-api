use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::EventId;

/// `GET` all matches for an event by [`EventId`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into))]
pub struct EventMatches<'a> {
    #[doc = "An event id. Example: `\"5f35882d53fbbb5894b43040\"`"]
    id: EventId<'a>,
}

impl EventMatches<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> EventMatchesBuilder<'a> {
        EventMatchesBuilder::default()
    }
}

impl<'a> Endpoint for EventMatches<'a> {
    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("/events/{}/matches", self.id).into()
    }
}
