use super::{common, DeviceType, Thing};

pub struct Generator {
    thing: Thing,
}

pub enum GeneratorState {
    Off,
    Priming,
    Starting,
    Running,
}

impl GeneratorState {
    pub fn to_string(&self) -> String {
        match self {
            GeneratorState::Off => "OFF".to_string(),
            GeneratorState::Priming => "PRIMING".to_string(),
            GeneratorState::Starting => "STARTING".to_string(),
            GeneratorState::Running => "RUNNING".to_string(),
        }
    }

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

#[derive(Debug)]
pub enum GeneratorStateConversionError {
    UnknownValue(String),
}

impl std::fmt::Display for GeneratorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Generator {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Generator));
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
        let _ = common::set_field(&mut self.thing, "command", "ON".to_string()).await;
    }

    pub async fn off(&mut self) {
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "command", "OFF".to_string()).await;
    }

    pub async fn state(&self) -> Result<GeneratorState, GeneratorStateConversionError> {
        let string = common::get_field(&self.thing, "command").await;
        GeneratorState::from_string(string)
    }
}
