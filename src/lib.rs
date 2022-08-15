mod common;
mod dimmer;
mod generator;
mod hvac;
mod sse;
mod switch;
mod tank;

pub use common::{OnlineState, OnlineStateConversionError};
pub use dimmer::Dimmer;
pub use generator::{Generator, GeneratorState, GeneratorStateConversionError};
pub use hvac::{HvacFan, HvacFanConversionError, HvacMode, HvacModeConversionError, HVAC};
pub use sse::{Channel, Configuration, DeviceType, LinkState, StatusInfo, Thing};
pub use switch::{Switch, SwitchState, SwitchStateConversionError};
pub use tank::Tank;

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
