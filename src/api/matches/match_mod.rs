use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::MatchId;

/// `GET` a single match by its [`MatchId`]
#[derive(Debug, PartialEq, Eq, Builder, Clone)]
#[builder(setter(into))]
pub struct Match<'a> {
    #[doc = "A match ID. Example: `\"6043152fa09e7fba40d2ae62\"`"]
    id: MatchId<'a>,
}

impl Match<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> MatchBuilder<'a> {
        MatchBuilder::default()
    }
}

impl Endpoint for Match<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/matches/{}", self.id).into()
    }
}
