#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let mut switchs = things
        .into_iter()
        .filter(|thing| thing.label() == "Water Pump")
        .collect::<Vec<_>>();
    let mut switch =
        lci_gateway::Switch::new(switchs.remove(0)).expect("Failed to convert to switch");
    println!(
        "{} = {}",
        switch.label(),
        switch.online().await.expect("Failed to get online status")
    );
    println!(
        "Fault = {}",
        switch.fault().await.expect("Failed to get fault status")
    );
    println!("Relay current = {}", switch.relay_current().await);
    println!(
        "state = {}",
        switch.state().await.expect("Failed to get state")
    );
    println!("Turning on");
    switch.on().await;
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    println!(
        "state = {}",
        switch.state().await.expect("Failed to get state")
    );
    println!("Relay current = {}", switch.relay_current().await);
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    println!("Turning off");
    switch.off().await;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!(
        "state = {}",
        switch.state().await.expect("Failed to get state")
    );
    println!("Relay current = {}", switch.relay_current().await);
}
