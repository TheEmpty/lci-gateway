use super::{common, DeviceType, Thing};
use thiserror::Error;

/// A connected generator.
pub struct Generator {
    thing: Thing,
}

/// The generator's detected discrete state.
#[derive(Debug, PartialEq, Eq)]
pub enum GeneratorState {
    /// The generator is not.
    Off,
    /// The generator is priming gas.
    Priming,
    /// The generator is cranking.
    Starting,
    /// The generator is active and running.
    Running,
}

impl GeneratorState {
    /// Returns generator enum value from the string value.
    /// Accepted values: ["OFF", "PRIMING", "STARTING", "RUNNING"]
    pub fn from_string(string: String) -> Result<Self, GeneratorStateConversionError> {
        match string.to_uppercase().as_str() {
            "OFF" => Ok(GeneratorState::Off),
            "PRIMING" => Ok(GeneratorState::Priming),
            "STARTING" => Ok(GeneratorState::Starting),
            "RUNNING" => Ok(GeneratorState::Running),
            _ => Err(GeneratorStateConversionError::UnknownValue(string)),
        }
    }
}

impl Generator {
    /// Create a new Generator from a generic "Thing". LCI gateway must publish the type as a Generator.
    pub fn new(thing: Thing) -> Result<Self, GeneratorError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Generator) {
            Ok(Self { thing })
        } else {
            Err(GeneratorError::InvalidDeviceType(thing_type))
        }
    }

    /// Returns the label of the device such as "Generator".
    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    /// Sends the command to start the generator.
    /// The LCI system will automatically prime and attempt to start the generator.
    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "command", "ON".to_string()).await?;
        Ok(())
    }

    /// Sends the command to turn off the generator.
    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "command", "OFF".to_string()).await?;
        Ok(())
    }

    /// Gets the current state of the generator.
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

/// Returned when a Generator can not be made from the given "thing".
#[derive(Debug, Error)]
pub enum GeneratorError {
    /// Returned when the "thing" is not a generator.
    #[error("The device type {0:?} is not a generator.")]
    InvalidDeviceType(Option<DeviceType>),
}

/// The generator state could not be fetched.
#[derive(Debug, Error)]
pub enum GeneratorStateConversionError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{0}' could not be converted to a GeneratorState.")]
    UnknownValue(String),
}
