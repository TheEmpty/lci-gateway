use super::{common, DeviceType, Thing};

pub struct Switch {
    thing: Thing,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SwitchState {
    Off,
    On,
}

impl SwitchState {
    pub fn from_string(string: String) -> Result<Self, SwitchStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(SwitchState::Off),
            "ON" => Ok(SwitchState::On),
            _ => Err(SwitchStateConversionError::UnknownValue(string)),
        }
    }
}

impl Switch {
    pub fn new(thing: Thing) -> Result<Self, SwitchError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Switch) {
            Ok(Self { thing })
        } else {
            Err(SwitchError::InvalidDeviceType(thing_type))
        }
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "switch", "ON".to_string()).await?;
        Ok(())
    }

    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "switch", "OFF".to_string()).await?;
        Ok(())
    }

    pub async fn state(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "switch").await?;
        SwitchState::from_string(string)
    }

    pub async fn fault(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "fault").await?;
        SwitchState::from_string(string)
    }

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

#[derive(Debug)]
pub enum SwitchError {
    InvalidDeviceType(Option<DeviceType>),
}

#[derive(Debug)]
pub enum SwitchStateConversionError {
    UnknownValue(String),
    GetFailure(common::GetFailure),
}

#[derive(Debug)]
pub enum SwitchRelayCurrentError {
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
