use chrono::{TimeZone, Utc};
use octanezsr_api::{
    api::{
        events::{Event, EventMatches, EventParticipants, EventSorting, ListEvents},
        AsyncQuery, Collection, Direction,
    },
    types, OctaneZsrBuilder, PagedEndpointExt,
};

use futures::{StreamExt, TryStreamExt};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    // Search for events with "RLCS" in the name
    let endpoint = ListEvents::builder().name("RLCS").build().unwrap();
    endpoint
        .stream(&client)
        .try_for_each_concurrent(2, |event: types::Event| async move {
            Ok(println!("{:?}", event.name))
        })
        .await?;

    // Search for events after October 21 2020. Sorted by tier, descending.
    let date = Utc.ymd(2020, 10, 21).and_hms(0, 0, 0);
    let endpoint = ListEvents::builder()
        .after(date)
        .sort(EventSorting::Tier, Direction::Desc)
        .build()
        .unwrap();
    endpoint
        .stream(&client)
        .take(20)
        .try_for_each_concurrent(4, |event: types::Event| async move {
            Ok(println!(
                "{}: {:?}, {}",
                event.name.unwrap_or_default(),
                event.tier,
                event
                    .start_date
                    .map(|date| date.to_rfc2822())
                    .unwrap_or_default()
            ))
        })
        .await?;

    // Get a single event using the id "5f35882d53fbbb5894b43040"
    let endpoint = Event::builder()
        .id("5f35882d53fbbb5894b43040")
        .build()
        .unwrap();
    let event: types::Event = endpoint.query_async(&client).await?;
    println!("{}", event.name.unwrap_or_else(|| "Missing Name".into()));

    // Get all participants for the event with id "5f35882d53fbbb5894b43040"
    let endpoint = EventParticipants::builder()
        .id("5f35882d53fbbb5894b43040")
        .build()
        .unwrap();
    let participants: Collection<types::Participant> = endpoint.query_async(&client).await?;
    let participants = participants.into_inner();
    for participant in participants {
        let players: Vec<_> = participant.players.into_iter().map(|p| p.tag).collect();
        println!("{}: {}", participant.team.name, players.join(","));
    }

    // Get all matches for the event with id "5f35882d53fbbb5894b43040"
    let endpoint = EventMatches::builder()
        .id("5f35882d53fbbb5894b43040")
        .build()
        .unwrap();
    let matches: Collection<types::Match> = endpoint.query_async(&client).await?;
    let matches = matches.into_inner();
    println!(
        "{} vs {}",
        matches[0]
            .blue
            .as_ref()
            .and_then(|side| side.team.as_ref().map(|ti| ti.team.name.clone()))
            .unwrap_or_else(|| "Blue".into()),
        matches[0]
            .orange
            .as_ref()
            .and_then(|side| side.team.as_ref().map(|ti| ti.team.name.clone()))
            .unwrap_or_else(|| "Orange".into())
    );

    Ok(())
}
