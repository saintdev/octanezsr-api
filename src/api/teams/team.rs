use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

use super::TeamId;

/// `GET` a single team by its [`TeamId`]
#[derive(Debug, PartialEq, Eq, Builder, Clone)]
#[builder(setter(into))]
pub struct Team<'a> {
    #[doc = "A team ID. Example: `\"6020bc70f1e4807cc70023c7\"`"]
    id: TeamId<'a>,
}

impl Team<'_> {
    /// Create a builder for this endpoint
    pub fn builder<'a>() -> TeamBuilder<'a> {
        TeamBuilder::default()
    }
}

impl Endpoint for Team<'_> {
    fn endpoint(&self) -> Cow<'static, str> {
        format!("/teams/{}", self.id).into()
    }
}
