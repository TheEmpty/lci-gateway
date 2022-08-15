#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let tanks = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(10.0))
        .collect::<Vec<_>>();

    for tank in tanks {
        let tank = lci_gateway::Tank::new(tank).expect("Failed to convert to a tank");
        println!(
            "{} [{}] = {}%",
            tank.label(),
            tank.online().await.expect("Failed to get online state"),
            tank.level().await
        );
    }
}
