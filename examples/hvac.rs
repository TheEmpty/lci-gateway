#[tokio::main]
async fn main() {
    env_logger::init();
    print_status().await;
    control_fan().await;
}

async fn control_fan() {
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");
    let hvacs: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(16.0))
        .collect();
    // Turn the fans on High then Auto.
    for hvac in hvacs {
        let mut hvac = lci_gateway::HVAC::new(hvac).expect("Failed to get HVAC");
        println!("Setting {} fan to high.", hvac.label());
        hvac.set_fan(&lci_gateway::HvacFan::High).await;
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        println!("Setting {} fan to auto.", hvac.label());
        hvac.set_fan(&lci_gateway::HvacFan::Auto).await;
    }
}

async fn print_status() {
    let things = lci_gateway::get_things()
        .await
        .expect("Couldn't get things");

    let hvacs: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(16.0))
        .collect();

    // Display some friendly data
    for hvac in hvacs {
        let hvac = lci_gateway::HVAC::new(hvac).expect("Failed to get HVAC");
        println!("{} [{}]", hvac.label(), hvac.status().await);
        println!("  Inside temp: {}", hvac.inside_temprature().await);
        println!("  Outside temp: {}", hvac.outside_temprature().await);
        println!(
            "  {} from {} to {}",
            hvac.status().await,
            hvac.low_temp().await,
            hvac.high_temp().await
        );
        println!(
            "  Fan: {}",
            hvac.fan().await.expect("Failed to get fan state")
        );
    }
}
