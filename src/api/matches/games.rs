use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::MatchId;

/// `GET` all games for a match  by its [`MatchId`]
#[derive(Debug, PartialEq, Eq, Builder, Clone)]
#[builder(setter(into))]
pub struct MatchGames<'a> {
    #[doc = "A match id. Example: `\"6043152fa09e7fba40d2ae62\"`"]
    id: MatchId<'a>,
}

impl MatchGames<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> MatchGamesBuilder<'a> {
        MatchGamesBuilder::default()
    }
}

impl Endpoint for MatchGames<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/matches/{}/games", self.id).into()
    }
}
