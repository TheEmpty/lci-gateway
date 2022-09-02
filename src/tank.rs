use super::{common, DeviceType, Thing};
use thiserror::Error;

/// A connected tank.
pub struct Tank {
    thing: Thing,
}

impl Tank {
    /// Create a new Tank from a generic "Thing". LCI gateway must publish the type as a Tank.
    pub fn new(thing: Thing) -> Result<Self, TankError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Tank) {
            Ok(Self { thing })
        } else {
            Err(TankError::InvalidDeviceType(thing_type))
        }
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    /// Returns the label of the device such as "Fresh Water" or "Generator Fuel".
    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    /// Gets the current percentage of the tank. Note accuracy depends on your sensors.
    pub async fn level(&self) -> Result<common::Percentage, TankLevelError> {
        let string = common::get_field(&self.thing, "tank_level").await?;
        let val = string
            .parse::<u8>()
            .map_err(|e| TankLevelError::Parse(string, e))?;
        Ok(common::Percentage::new(val))
    }
}

/// Returned when a Tank can not be made from the given "thing".
#[derive(Debug, Error)]
pub enum TankError {
    /// Returned when the "thing" is not a tank.
    #[error("The device type {0:?} is not a tank.")]
    InvalidDeviceType(Option<DeviceType>),
}

/// The tank level could not be fetched.
#[derive(Debug, Error)]
pub enum TankLevelError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{1}' could not be converted to a percentage.")]
    Parse(String, std::num::ParseIntError),
}

impl From<common::GetFailure> for TankLevelError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}
