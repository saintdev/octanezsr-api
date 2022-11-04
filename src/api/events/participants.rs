use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::EventId;

/// `GET` all participants for an event by [`EventId`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into))]
pub struct EventParticipants<'a> {
    #[doc = "An event id. Example: `\"5f35882d53fbbb5894b43040\"`"]
    id: EventId<'a>,
}

impl EventParticipants<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> EventParticipantsBuilder<'a> {
        EventParticipantsBuilder::default()
    }
}

impl<'a> Endpoint for EventParticipants<'a> {
    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("/events/{}/participants", self.id).into()
    }
}
