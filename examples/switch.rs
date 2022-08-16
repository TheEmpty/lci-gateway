#[tokio::main]
async fn main() -> Result<(), DemoError> {
    env_logger::init();
    let things = lci_gateway::get_things().await?;

    let mut switchs = things
        .into_iter()
        .filter(|thing| thing.label() == "Water Pump")
        .collect::<Vec<_>>();
    let mut switch = lci_gateway::Switch::new(switchs.remove(0))?;
    println!("{} = {}", switch.label(), switch.online().await?);
    println!("Fault = {}", switch.fault().await?);
    println!("Relay current = {}", switch.relay_current().await?);
    println!("state = {}", switch.state().await?);
    println!("Turning on");
    switch.on().await?;
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    println!("state = {}", switch.state().await?);
    println!("Relay current = {}", switch.relay_current().await?);
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    println!("Turning off");
    switch.off().await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("state = {}", switch.state().await?);
    println!("Relay current = {}", switch.relay_current().await?);

    Ok(())
}

#[derive(Debug)]
enum DemoError {
    SwitchError(lci_gateway::SwitchError),
    ThingError(lci_gateway::ThingError),
    OnlineStateConversionError(lci_gateway::OnlineStateConversionError),
    SwitchStateConversionError(lci_gateway::SwitchStateConversionError),
    SwitchRelayCurrentError(lci_gateway::SwitchRelayCurrentError),
    SetError(lci_gateway::SetError),
}

impl From<lci_gateway::ThingError> for DemoError {
    fn from(error: lci_gateway::ThingError) -> Self {
        log::error!("ThingError: {:?}", error);
        Self::ThingError(error)
    }
}

impl From<lci_gateway::SwitchError> for DemoError {
    fn from(error: lci_gateway::SwitchError) -> Self {
        log::error!("SwitchError: {:?}", error);
        Self::SwitchError(error)
    }
}

impl From<lci_gateway::OnlineStateConversionError> for DemoError {
    fn from(error: lci_gateway::OnlineStateConversionError) -> Self {
        log::error!("OnlineStateConversionError: {:?}", error);
        Self::OnlineStateConversionError(error)
    }
}

impl From<lci_gateway::SwitchStateConversionError> for DemoError {
    fn from(error: lci_gateway::SwitchStateConversionError) -> Self {
        log::error!("SwitchStateConversionError: {:?}", error);
        Self::SwitchStateConversionError(error)
    }
}

impl From<lci_gateway::SwitchRelayCurrentError> for DemoError {
    fn from(error: lci_gateway::SwitchRelayCurrentError) -> Self {
        log::error!("SwitchRelayCurrentError: {:?}", error);
        Self::SwitchRelayCurrentError(error)
    }
}

impl From<lci_gateway::SetError> for DemoError {
    fn from(error: lci_gateway::SetError) -> Self {
        log::error!("SetError: {:?}", error);
        Self::SetError(error)
    }
}
