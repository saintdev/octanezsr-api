use octanezsr_api::{
    api::{
        matches::{ListMatches, Match, MatchGames},
        AsyncQuery, Collection,
    },
    types, OctaneZsrBuilder, PagedEndpointExt,
};

use futures::{StreamExt, TryStreamExt};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    // Get a list of all matches with player id "5f3d8fdd95f40596eae23d97"
    let endpoint = ListMatches::builder()
        .player("5f3d8fdd95f40596eae23d97")
        .build()
        .unwrap();
    let mut stream = endpoint.stream::<types::Match, _>(&client).take(200);
    while let Some(mtch) = stream.try_next().await? {
        let blue = mtch.blue.unwrap();
        let orange = mtch.orange.unwrap();
        println!(
            "{} vs {}: {}-{}",
            blue.team
                .map(|info| info.team.name)
                .unwrap_or_else(|| "Blue".into()),
            orange
                .team
                .map(|info| info.team.name)
                .unwrap_or_else(|| "Orange".into()),
            blue.score.unwrap_or_default(),
            orange.score.unwrap_or_default()
        );
    }

    // Get a single match with id "6043152fa09e7fba40d2ae62"
    let endpoint = Match::builder()
        .id("6043152fa09e7fba40d2ae62")
        .build()
        .unwrap();
    let m: types::Match = endpoint.query_async(&client).await?;
    let blue = m.blue.unwrap();
    let orange = m.orange.unwrap();
    println!(
        "{} vs {}: {}-{}",
        blue.team
            .map(|ti| ti.team.name)
            .unwrap_or_else(|| "Blue".into()),
        orange
            .team
            .map(|ti| ti.team.name)
            .unwrap_or_else(|| "Orange".into()),
        blue.score.unwrap_or_default(),
        blue.score.unwrap_or_default()
    );

    // Get games for a single match with id "6043152fa09e7fba40d2ae62"
    let endpoint = MatchGames::builder()
        .id("6043152fa09e7fba40d2ae62")
        .build()
        .unwrap();
    let games: Collection<types::Game> = endpoint.query_async(&client).await?;
    for game in games.into_inner() {
        println!(
            "{} - {}",
            game.blue.score.unwrap_or_default(),
            game.orange.score.unwrap_or_default()
        );
    }

    Ok(())
}
