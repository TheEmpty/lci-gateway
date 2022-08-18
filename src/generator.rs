use super::{common, DeviceType, Thing};

pub struct Generator {
    thing: Thing,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GeneratorState {
    Off,
    Priming,
    Starting,
    Running,
}

impl GeneratorState {
    pub fn from_string(string: String) -> Result<Self, GeneratorStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(GeneratorState::Off),
            "PRIMING" => Ok(GeneratorState::Priming),
            "STARTING" => Ok(GeneratorState::Starting),
            "RUNNING" => Ok(GeneratorState::Running),
            _ => Err(GeneratorStateConversionError::UnknownValue(string)),
        }
    }
}

impl Generator {
    pub fn new(thing: Thing) -> Result<Self, GeneratorError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Generator) {
            Ok(Self { thing })
        } else {
            Err(GeneratorError::InvalidDeviceType(thing_type))
        }
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "command", "ON".to_string()).await?;
        Ok(())
    }

    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "command", "OFF".to_string()).await?;
        Ok(())
    }

    pub async fn state(&self) -> Result<GeneratorState, GeneratorStateConversionError> {
        let string = common::get_field(&self.thing, "state")
            .await
            .map_err(GeneratorStateConversionError::GetFailure)?;
        GeneratorState::from_string(string)
    }
}

impl std::fmt::Display for GeneratorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            GeneratorState::Off => "Off".to_string(),
            GeneratorState::Priming => "Priming".to_string(),
            GeneratorState::Starting => "Starting".to_string(),
            GeneratorState::Running => "Running".to_string(),
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug)]
pub enum GeneratorError {
    InvalidDeviceType(Option<DeviceType>),
}

#[derive(Debug)]
pub enum GeneratorStateConversionError {
    GetFailure(common::GetFailure),
    UnknownValue(String),
}
