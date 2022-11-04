use octanezsr_api::{api::teams::ListTeams, types, OctaneZsrBuilder, PagedEndpointExt};

use futures::TryStreamExt;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = OctaneZsrBuilder::new().build_async()?;

    let endpoint = ListTeams::builder().build().unwrap();

    let _matches: Vec<types::Team> = endpoint.stream(&client).try_collect().await?;

    Ok(())
}
