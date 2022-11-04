use octanezsr_api::{
    api::{
        games::{Game, ListGames},
        AsyncQuery,
    },
    types, OctaneZsrBuilder, PagedEndpointExt,
};

use futures::TryStreamExt;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    // Get a list of all games that have team with id "6020bc70f1e4807cc70023c7"
    let endpoint = ListGames::builder()
        .team("6020bc70f1e4807cc70023c7")
        .build()?;
    let games: Vec<types::Game> = endpoint.stream(&client).try_collect().await?;
    for game in games {
        println!("{}", game.id);
    }

    // Get a single game with id "6082fb4c0d9dcf9da5a4d2ea"
    let endpoint = Game::builder().id("6082fb4c0d9dcf9da5a4d2ea").build()?;
    let game: types::Game = endpoint.query_async(&client).await?;
    println!("{}", game.id);

    Ok(())
}
