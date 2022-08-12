#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things().await.expect("Couldn't get things");

    let mut switchs = things.into_iter().filter(|thing| thing.label() == "Water Pump").collect::<Vec<_>>();
    let switch = lci_gateway::Switch::new(switchs.remove(0)).expect("Failed to convert to switch");
    switch.on().await;
    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
    switch.off().await;
}
