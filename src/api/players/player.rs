use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::PlayerId;

/// `GET` a single player by its [`PlayerId`]
#[derive(Debug, PartialEq, Eq, Builder, Clone)]
#[builder(setter(into))]
pub struct Player<'a> {
    #[doc = "A player ID. Example: `\"5f35882d53fbbb5894b43040\"`"]
    id: PlayerId<'a>,
}

impl Player<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> PlayerBuilder<'a> {
        PlayerBuilder::default()
    }
}

impl Endpoint for Player<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/players/{}", self.id).into()
    }
}
