#[tokio::main]
async fn main() -> Result<(), DemoError> {
    env_logger::init();
    let things = lci_gateway::get_things().await?;

    let tanks = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(10.0))
        .collect::<Vec<_>>();

    for tank in tanks {
        let tank = lci_gateway::Tank::new(tank).expect("Failed to convert to a tank");
        println!(
            "{} [{}] = {}",
            tank.label(),
            tank.online().await?,
            tank.level().await?
        );
    }
    Ok(())
}

#[derive(Debug)]
enum DemoError {
    ThingError(lci_gateway::ThingError),
    OnlineStateConversionError(lci_gateway::OnlineStateConversionError),
    TankLevelError(lci_gateway::TankLevelError),
}

impl From<lci_gateway::ThingError> for DemoError {
    fn from(error: lci_gateway::ThingError) -> Self {
        Self::ThingError(error)
    }
}

impl From<lci_gateway::OnlineStateConversionError> for DemoError {
    fn from(error: lci_gateway::OnlineStateConversionError) -> Self {
        Self::OnlineStateConversionError(error)
    }
}

impl From<lci_gateway::TankLevelError> for DemoError {
    fn from(error: lci_gateway::TankLevelError) -> Self {
        Self::TankLevelError(error)
    }
}
