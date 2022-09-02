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
    HvacError, HvacFanMode, HvacFanModeConversionError, HvacHighTemperatureFailure,
    HvacInsideTemperatureFailure, HvacLowTemperatureFailure, HvacMode, HvacModeConversionError,
    HvacOutsideTemperatureFailure, HvacStatus, HvacStatusConversionError, HvacStatusFailure, HVAC,
};
pub use sse::{Configuration, DeviceType, Thing};
pub use switch::{
    Switch, SwitchError, SwitchRelayCurrentError, SwitchState, SwitchStateConversionError,
};
pub use tank::{Tank, TankError, TankLevelError};
use thiserror::Error;

/// Used when the list of things can not be fetched.
#[derive(Debug, Error)]
pub enum ThingError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    Getting(reqwest::Error),
    /// Could not get the text from the HTTP response.
    #[error("The text could not be retrieved. {0}")]
    Text(reqwest::Error),
    /// The LCI gateway returned unexpected or invalid JSON.
    #[error("The JSON response could not be parsed. {0}")]
    ConvertingJson(serde_json::Error),
}

/// Returns the "things" availabe in the LCI Gateway.
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
            ThingError::Text(err)
        })?;
    log::trace!("Converting with serde.");
    let things: Vec<Thing> = serde_json::from_str(&body).map_err(ThingError::ConvertingJson)?;
    log::trace!("returning things");
    Ok(things)
}
