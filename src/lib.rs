mod common;
mod dimmer;
mod generator;
mod hvac;
mod sse;
mod switch;
mod tank;

pub use common::{OnlineState, OnlineStateConversionError, Percentage, SetError};
pub use dimmer::{Dimmer, DimmerBrightnessError, DimmerError, SetBrightnessError};
pub use generator::{Generator, GeneratorError, GeneratorState, GeneratorStateConversionError};
pub use hvac::{
    HvacError, HvacFan, HvacFanConversionError, HvacHighTemperatureFailure,
    HvacInsideTemperatureFailure, HvacLowTemperatureFailure, HvacMode, HvacModeConversionError,
    HvacOutsideTemperatureFailure, HvacStatus, HvacStatusConversionError, HvacStatusFailure, HVAC,
};
pub use sse::{Channel, Configuration, DeviceType, LinkState, StatusInfo, Thing};
pub use switch::{
    Switch, SwitchError, SwitchRelayCurrentError, SwitchState, SwitchStateConversionError,
};
pub use tank::{Tank, TankError, TankLevelError};

#[derive(Debug)]
pub enum ThingError {
    Getting(reqwest::Error),
    PullingText(reqwest::Error),
    ConvertingJson(serde_json::Error),
}

pub async fn get_things() -> Result<Vec<Thing>, ThingError> {
    log::trace!("Fetching things");
    let body = reqwest::get("http://192.168.1.4:8080/rest/things/")
        .await
        .map_err(|err| {
            log::error!("Failed to request {:?}", err);
            ThingError::Getting(err)
        })?
        .text()
        .await
        .map_err(|err| {
            log::error!("Failed to pull text {:?}", err);
            ThingError::PullingText(err)
        })?;
    log::trace!("Converting with serde.");
    let things: Vec<Thing> = serde_json::from_str(&body).map_err(ThingError::ConvertingJson)?;
    log::trace!("returning things");
    Ok(things)
}
