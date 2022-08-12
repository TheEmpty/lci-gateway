#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let futures: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(20.0))
        .map(|thing| blink_dimmer(thing))
        .collect();

    futures::future::join_all(futures).await;
}

async fn blink_dimmer(thing: lci_gateway::Thing) {
    let dimmer = lci_gateway::Dimmer::new(thing).expect("Failed to convert to dimmer");

    // on
    dimmer.on().await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await);

    // off
    dimmer.off().await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await);

    // on @ 50%
    dimmer.set_brightness(50).await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await);

    // on 100
    dimmer.set_brightness(100).await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await);
}
