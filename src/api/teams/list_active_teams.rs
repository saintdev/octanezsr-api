use std::borrow::Cow;

use derive_builder::Builder;

use crate::api::endpoint::Endpoint;

/// `GET` a list of active teams
#[derive(Default, Debug, Builder, Clone)]
#[builder(default, setter(into, strip_option))]
pub struct ListActiveTeams {}

impl ListActiveTeams {
    /// Create a builder for this endpoint
    pub fn builder() -> ListActiveTeamsBuilder {
        ListActiveTeamsBuilder::default()
    }
}

impl Endpoint for ListActiveTeams {
    fn endpoint(&self) -> Cow<'static, str> {
        "/teams/active".into()
    }
}
