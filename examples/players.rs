use octanezsr_api::{
    api::{
        players::{ListPlayers, Player},
        AsyncQuery,
    },
    types, OctaneZsrBuilder, PagedEndpointExt,
};

use futures::TryStreamExt;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    // Get a list of players with "Kro" in their tag
    let endpoint = ListPlayers::builder().tag("Kro").build().unwrap();
    let mut stream = endpoint.stream::<types::Player, _>(&client);
    while let Some(player) = stream.try_next().await? {
        println!("{}: {}", player.tag, player.id);
    }

    // Get a single player with id "5f35882d53fbbb5894b43040"
    let endpoint = Player::builder().id("5f35882d53fbbb5894b43040").build()?;
    let player: types::Player = endpoint.query_async(&client).await?;
    println!("{}", player.tag);

    Ok(())
}
