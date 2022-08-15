use super::{common, DeviceType, Thing};

pub struct Switch {
    thing: Thing,
}

pub enum SwitchState {
    On,
    Off,
}

impl SwitchState {
    pub fn to_string(&self) -> String {
        match self {
            SwitchState::Off => "OFF".to_string(),
            SwitchState::On => "ON".to_string(),
        }
    }

    pub fn from_string(string: String) -> Result<Self, SwitchStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(SwitchState::Off),
            "ON" => Ok(SwitchState::On),
            _ => Err(SwitchStateConversionError::UnknownValue(string)),
        }
    }
}

impl std::fmt::Display for SwitchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub enum SwitchStateConversionError {
    UnknownValue(String),
}

impl Switch {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Switch));
        Ok(Self { thing })
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn on(&mut self) {
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "switch", "ON".to_string()).await;
    }

    pub async fn off(&mut self) {
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "switch", "OFF".to_string()).await;
    }

    pub async fn state(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "switch").await;
        SwitchState::from_string(string)
    }

    pub async fn fault(&self) -> Result<SwitchState, SwitchStateConversionError> {
        let string = common::get_field(&self.thing, "fault").await;
        SwitchState::from_string(string)
    }

    pub async fn relay_current(&self) -> String {
        common::get_field(&self.thing, "current").await
    }
}
