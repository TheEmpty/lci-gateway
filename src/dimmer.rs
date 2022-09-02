use super::{common, DeviceType, Thing};
use thiserror::Error;

/// A connected light that can go between 0 and 100 percent brightness.
pub struct Dimmer {
    thing: Thing,
}

impl Dimmer {
    /// Create a new Dimmer from a generic "Thing". LCI gateway must publish the type as a Dimmer.
    pub fn new(thing: Thing) -> Result<Self, DimmerError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Dimmer) {
            Ok(Self { thing })
        } else {
            Err(DimmerError::InvalidDeviceType(thing_type))
        }
    }

    /// Returns the label of the device such as "Kitchen Lights".
    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    /// Returns the dimmer's brightness level.
    pub async fn brightness(&self) -> Result<common::Percentage, DimmerBrightnessError> {
        let string = common::get_field(&self.thing, "dimmer")
            .await
            .map_err(DimmerBrightnessError::GetFailure)?;
        let val = string
            .parse::<u8>()
            .map_err(|e| DimmerBrightnessError::Parse(string, e))?;
        Ok(common::Percentage::new(val))
    }

    /// Turns on the dimmer to 100%
    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "dimmer", "ON".to_string()).await?;
        Ok(())
    }

    /// Turns off the dimmer and sets to 0%
    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "dimmer", "OFF".to_string()).await?;
        Ok(())
    }

    /// Turns on the dimmer and sets the brightness between 0 and 100.
    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), SetBrightnessError> {
        if brightness > 100 {
            return Err(SetBrightnessError::InvalidValue(brightness));
        }
        common::set_field(&mut self.thing, "dimmer", brightness.to_string()).await?;
        Ok(())
    }
}

/// Returned when a Dimmer can not be made from the given "thing".
#[derive(Debug, Error)]
pub enum DimmerError {
    /// Returned when the "thing" is not a dimmer.
    #[error("The device type {0:?} is not a dimmer.")]
    InvalidDeviceType(Option<DeviceType>),
}

/// Returned when getting the brightness fails.
#[derive(Debug, Error)]
pub enum DimmerBrightnessError {
    /// The LCI gateway could not be reached.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseIntError),
}

/// The brightness of the dimmer could not be set.
#[derive(Debug, Error)]
pub enum SetBrightnessError {
    /// The given value is not between 0 and 100
    #[error("Supplied brightness {0} is not between 0 and 100.")]
    InvalidValue(u8),
    /// The LCI gateway request failed.
    #[error("The set command failed to process. {0}")]
    SetError(common::SetError),
}

impl From<common::SetError> for SetBrightnessError {
    fn from(error: common::SetError) -> Self {
        Self::SetError(error)
    }
}
