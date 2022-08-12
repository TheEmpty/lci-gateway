#[tokio::main]
async fn main() {
    env_logger::init();
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let hvacs: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(16.0))
        .collect();

    for hvac in hvacs {
        let hvac = lci_gateway::HVAC::new(hvac).expect("Failed to get HVAC");
        println!("{} [{}]", hvac.label(), hvac.status().await);
        println!("  Inside temp: {}", hvac.inside_temprature().await);
        println!("  Outside temp: {}", hvac.outside_temprature().await);
    }
}
