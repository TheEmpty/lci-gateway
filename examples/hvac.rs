#[tokio::main]
async fn main() -> Result<(), DemoError> {
    env_logger::init();
    print_status().await?;
    control_fan().await?;
    Ok(())
}

async fn control_fan() -> Result<(), FanDemoError> {
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
        hvac.set_fan(&lci_gateway::HvacFanMode::High).await?;
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        println!("Setting {} fan to auto.", hvac.label());
        hvac.set_fan(&lci_gateway::HvacFanMode::Auto).await?;
    }

    Ok(())
}

async fn print_status() -> Result<(), StatusDemoError> {
    let things = lci_gateway::get_things().await?;

    let hvacs: Vec<_> = things
        .into_iter()
        .filter(|thing| thing.configuration().deviceType() == &Some(16.0))
        .collect();

    // Display some friendly data
    for hvac in hvacs {
        let hvac = lci_gateway::HVAC::new(hvac).expect("Failed to get HVAC");
        println!("{} [{}]", hvac.label(), hvac.status().await?);
        println!("  Status: {}", hvac.status().await?);
        println!("  Inside temp: {}", hvac.inside_temperature().await?);
        println!("  Outside temp: {}", hvac.outside_temperature().await?);
        println!(
            "  {} from {} to {}",
            hvac.status().await?,
            hvac.low_temperature().await?,
            hvac.high_temperature().await?
        );
        println!("  Fan: {}", hvac.fan().await?);
    }

    Ok(())
}

#[derive(Debug)]
enum DemoError {
    StatusDemoError(StatusDemoError),
    FanDemoError(FanDemoError),
}

#[derive(Debug)]
enum FanDemoError {
    SetError(lci_gateway::SetError),
}

#[derive(Debug)]
enum StatusDemoError {
    ThingError(lci_gateway::ThingError),
    HvacStatusFailure(lci_gateway::HvacStatusFailure),
    HvacInsideTemperatureFailure(lci_gateway::HvacInsideTemperatureFailure),
    HvacOutsideTemperatureFailure(lci_gateway::HvacOutsideTemperatureFailure),
    HvacLowTemperatureFailure(lci_gateway::HvacLowTemperatureFailure),
    HvacHighTemperatureFailure(lci_gateway::HvacHighTemperatureFailure),
    HvacFanModeConversionError(lci_gateway::HvacFanModeConversionError),
    HvacStatusConversionError(lci_gateway::HvacStatusConversionError),
}

impl From<FanDemoError> for DemoError {
    fn from(error: FanDemoError) -> Self {
        log::error!("FanDemoError: {:?}", error);
        Self::FanDemoError(error)
    }
}

impl From<StatusDemoError> for DemoError {
    fn from(error: StatusDemoError) -> Self {
        log::error!("StatusDemoError: {:?}", error);
        Self::StatusDemoError(error)
    }
}

impl From<lci_gateway::ThingError> for StatusDemoError {
    fn from(error: lci_gateway::ThingError) -> Self {
        log::error!("ThingError: {:?}", error);
        Self::ThingError(error)
    }
}

impl From<lci_gateway::HvacStatusFailure> for StatusDemoError {
    fn from(error: lci_gateway::HvacStatusFailure) -> Self {
        log::error!("HvacStatusFailure: {:?}", error);
        Self::HvacStatusFailure(error)
    }
}

impl From<lci_gateway::HvacInsideTemperatureFailure> for StatusDemoError {
    fn from(error: lci_gateway::HvacInsideTemperatureFailure) -> Self {
        log::error!("HvacInsideTemperatureFailure: {:?}", error);
        Self::HvacInsideTemperatureFailure(error)
    }
}

impl From<lci_gateway::HvacOutsideTemperatureFailure> for StatusDemoError {
    fn from(error: lci_gateway::HvacOutsideTemperatureFailure) -> Self {
        log::error!("HvacOutsideTemperatureFailure: {:?}", error);
        Self::HvacOutsideTemperatureFailure(error)
    }
}

impl From<lci_gateway::HvacLowTemperatureFailure> for StatusDemoError {
    fn from(error: lci_gateway::HvacLowTemperatureFailure) -> Self {
        log::error!("HvacLowTemperatureFailure: {:?}", error);
        Self::HvacLowTemperatureFailure(error)
    }
}

impl From<lci_gateway::HvacStatusConversionError> for StatusDemoError {
    fn from(error: lci_gateway::HvacStatusConversionError) -> Self {
        log::error!("HvacStatusConversionError {:?}", error);
        Self::HvacStatusConversionError(error)
    }
}

impl From<lci_gateway::HvacHighTemperatureFailure> for StatusDemoError {
    fn from(error: lci_gateway::HvacHighTemperatureFailure) -> Self {
        log::error!("HvacHighTemperatureFailure: {:?}", error);
        Self::HvacHighTemperatureFailure(error)
    }
}

impl From<lci_gateway::HvacFanModeConversionError> for StatusDemoError {
    fn from(error: lci_gateway::HvacFanModeConversionError) -> Self {
        log::error!("HvacFanModeConversionError: {:?}", error);
        Self::HvacFanModeConversionError(error)
    }
}

impl From<lci_gateway::SetError> for FanDemoError {
    fn from(error: lci_gateway::SetError) -> Self {
        log::error!("SetError: {:?}", error);
        Self::SetError(error)
    }
}
