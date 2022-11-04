use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::EventId;

/// `GET` a single event by its [`EventId`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into))]
pub struct Event<'a> {
    #[doc = "An event ID. Example: `\"5f35882d53fbbb5894b43040\"`"]
    id: EventId<'a>,
}

impl Event<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> EventBuilder<'a> {
        EventBuilder::default()
    }
}

impl<'a> Endpoint for Event<'a> {
    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("/events/{}", self.id).into()
    }
}
