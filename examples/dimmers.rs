#[tokio::main]
async fn main() -> Result<(), DemoError> {
    env_logger::init();
    let things = lci_gateway::get_things().await?;

    let futures: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(20.0))
        .map(|thing| dimmer_demo(thing))
        .collect();

    futures::future::join_all(futures).await;
    Ok(())
}

async fn dimmer_demo(thing: lci_gateway::Thing) -> Result<(), DimmerDemoError> {
    let mut dimmer = lci_gateway::Dimmer::new(thing)?;

    println!("{} = {}", dimmer.label(), dimmer.online().await?);

    // on
    dimmer.on().await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await?);

    // off
    dimmer.off().await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await?);

    // on @ 50%
    dimmer.set_brightness(50).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await?);

    // on 100
    dimmer.set_brightness(100).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("{} = {}", dimmer.label(), dimmer.brightness().await?);

    Ok(())
}

#[derive(Debug)]
enum DemoError {
    ThingError(lci_gateway::ThingError),
    DimmerDemoError(DimmerDemoError),
}

#[derive(Debug)]
enum DimmerDemoError {
    BrightnessError(lci_gateway::DimmerBrightnessError),
    OnlineError(lci_gateway::OnlineStateConversionError),
    DimmerError(lci_gateway::DimmerError),
    SetError(lci_gateway::SetError),
    SetBrightnessError(lci_gateway::SetBrightnessError),
}

impl From<DimmerDemoError> for DemoError {
    fn from(error: DimmerDemoError) -> Self {
        log::error!("DimmerDemoError: {:?}", error);
        Self::DimmerDemoError(error)
    }
}

impl From<lci_gateway::ThingError> for DemoError {
    fn from(error: lci_gateway::ThingError) -> Self {
        log::error!("ThingError: {:?}", error);
        Self::ThingError(error)
    }
}

impl From<lci_gateway::DimmerBrightnessError> for DimmerDemoError {
    fn from(error: lci_gateway::DimmerBrightnessError) -> Self {
        log::error!("DimmerBrightnessError: {:?}", error);
        Self::BrightnessError(error)
    }
}

impl From<lci_gateway::OnlineStateConversionError> for DimmerDemoError {
    fn from(error: lci_gateway::OnlineStateConversionError) -> Self {
        log::error!("OnlineStateConversionError: {:?}", error);
        Self::OnlineError(error)
    }
}

impl From<lci_gateway::DimmerError> for DimmerDemoError {
    fn from(error: lci_gateway::DimmerError) -> Self {
        log::error!("DimmerError: {:?}", error);
        Self::DimmerError(error)
    }
}

impl From<lci_gateway::SetError> for DimmerDemoError {
    fn from(error: lci_gateway::SetError) -> Self {
        log::error!("SetError: {:?}", error);
        Self::SetError(error)
    }
}

impl From<lci_gateway::SetBrightnessError> for DimmerDemoError {
    fn from(error: lci_gateway::SetBrightnessError) -> Self {
        log::error!("SetBrightnessError: {:?}", error);
        Self::SetBrightnessError(error)
    }
}
