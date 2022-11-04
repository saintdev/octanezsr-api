use octanezsr_api::{
    api::{
        teams::{ListActiveTeams, ListTeams, Team},
        AsyncQuery, Collection,
    },
    types, OctaneZsrBuilder, PagedEndpointExt,
};

use futures::TryStreamExt;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    // Get a list of teams with "Flip" in their name
    let endpoint = ListTeams::builder().name("Flip").build()?;
    let teams: Vec<types::Team> = endpoint.stream(&client).try_collect().await?;
    for team in teams {
        println!("{}", team.name);
    }

    // Get a collection of active teams
    let endpoint = ListActiveTeams::builder().build()?;
    let teams: Collection<types::Team> = endpoint.query_async(&client).await?;
    for team in teams.into_inner() {
        println!("{}", team.name);
    }

    // Get a single team with id "6020bc70f1e4807cc70023c7"
    let endpoint = Team::builder().id("6020bc70f1e4807cc70023c7").build()?;
    let team: types::Team = endpoint.query_async(&client).await?;
    println!("{}", team.name);

    Ok(())
}
