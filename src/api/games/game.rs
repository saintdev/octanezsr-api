use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::GameId;

/// `GET` a single game by its [`GameId`]
#[derive(Debug, PartialEq, Eq, Builder, Clone)]
#[builder(setter(into))]
pub struct Game<'a> {
    #[doc = "id for this game"]
    id: GameId<'a>,
}

impl Game<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> GameBuilder<'a> {
        GameBuilder::default()
    }
}

impl Endpoint for Game<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/games/{}", self.id).into()
    }
}
