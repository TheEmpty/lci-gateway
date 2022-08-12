#[tokio::main]
async fn main() {
    env_logger::init();
    let things = not_lci::get_things().await.expect("Couldn't get things");

    let futures: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(20.0))
        .map(|thing| blink_dimmer(thing))
        .collect();

    futures::future::join_all(futures).await;
}

async fn blink_dimmer(thing: not_lci::Thing) {
    let client = reqwest::Client::new();

    let dimmer = not_lci::Dimmer::new(thing).expect("Failed to convert to dimmer");
    dimmer.on().await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    dimmer.off().await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    dimmer.on().await;
}
