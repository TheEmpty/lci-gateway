use super::{common, DeviceType, Thing};
use thiserror::Error;

/// For things that go on and off.
/// Ex: lights, water pump, water heater, etc.
pub struct Switch {
    thing: Thing,
}

/// The only two states of a switch.
#[derive(Debug, PartialEq, Eq)]
pub enum SwitchState {
    Off,
    On,
}

impl SwitchState {
    /// Returns SwitchState enum value from the string value.
    /// Accepted values: ["OFF", "ON"]
    pub fn from_string(string: String) -> Result<Self, SwitchStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(SwitchState::Off),
            "ON" => Ok(SwitchState::On),
            _ => Err(SwitchStateConversionError::UnknownValue(string)),
        }
    }
}

impl Switch {
    /// Create a new Switch from a generic "Thing". LCI gateway must publish the type as a Switch.
    pub fn new(thing: Thing) -> Result<Self, SwitchError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Switch) {
            Ok(Self { thing })
        } else {
            Err(SwitchError::InvalidDeviceType(thing_type))
        }
    }

    /// Returns the label of the device such as "Water Pump" or "Kitchen Lights".
    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    /// Turns the relay/switch on.
    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "switch", "ON".to_string()).await?;
        Ok(())
    }

    /// Turns the relay/switch off.
    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "switch", "OFF".to_string()).await?;
        Ok(())
    }

    /// Gets the current relay state.
    pub async fn state(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "switch").await?;
        SwitchState::from_string(string)
    }

    /// Gets if the fault state is on or off.
    pub async fn fault(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "fault").await?;
        SwitchState::from_string(string)
    }

    /// Gets the current being used by the relay.
    pub async fn relay_current(&self) -> Result<String, SwitchRelayCurrentError> {
        Ok(common::get_field(&self.thing, "current").await?)
    }
}

impl std::fmt::Display for SwitchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            SwitchState::Off => "Off".to_string(),
            SwitchState::On => "On".to_string(),
        };
        write!(f, "{}", string)
    }
}

/// Returned when a Switch can not be made from the given "thing".
#[derive(Debug, Error)]
pub enum SwitchError {
    /// Returned when the "thing" is not a Switch.
    #[error("The device type {0:?} is not a Switch.")]
    InvalidDeviceType(Option<DeviceType>),
}

/// The switch state could not be fetched.
#[derive(Debug, Error)]
pub enum SwitchStateConversionError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{0}' could not be converted to a SwitchState.")]
    UnknownValue(String),
}

/// The switch relay usage could not be fetched.
#[derive(Debug, Error)]
pub enum SwitchRelayCurrentError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
}

impl From<common::GetFailure> for SwitchStateConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for SwitchRelayCurrentError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}
