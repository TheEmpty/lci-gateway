use super::{common, DeviceType, Thing};
use thiserror::Error;

/// A connected A/C
pub struct HVAC {
    thing: Thing,
}

/// Possible HVAC fan settings.
#[derive(Debug, PartialEq, Eq)]
pub enum HvacFanMode {
    /// Let the LCI or A/C decide fan level.
    Auto,
    /// The fan is manually set to low.
    Low,
    /// The fan is manually set to high.
    High,
}

/// Posible states for the A/C system.
#[derive(Debug, PartialEq, Eq)]
pub enum HvacMode {
    /// The A/C is not requested.
    Off,
    /// The A/C is only requested to heat based off set temperatures.
    Heat,
    /// The A/C is only requested to cool based off set temperatures.
    Cool,
    /// The A/C is only requested to heat or cool based off set temperatures.
    HeatCool,
}

/// The current state of the A/C HVAC. If it is actively cooling, failing, etc.
#[derive(Debug, PartialEq, Eq)]
pub enum HvacStatus {
    Off,
    Idle,
    Cooling,
    HeatPump,
    ElectricFurnace,
    GasFurnace,
    GasOverride,
    DeadTime,
    LoadShedding,
    FailOff,
    FailIdle,
    FailCooling,
    FailHeatPump,
    FailElectricFurnace,
    FailGasFurnace,
    FailGasOverride,
    FailDeadTime,
    FailShedding,
}

impl HvacFanMode {
    /// Returns HvacFan enum value from the string value.
    /// Accepted values: ["AUTO", "LOW", "HIGH"]
    pub fn from_string(string: String) -> Result<Self, HvacFanModeConversionError> {
        match string.to_uppercase().as_str() {
            "AUTO" => Ok(HvacFanMode::Auto),
            "LOW" => Ok(HvacFanMode::Low),
            "HIGH" => Ok(HvacFanMode::High),
            _ => Err(HvacFanModeConversionError::UnknownValue(string)),
        }
    }
}

impl HvacMode {
    /// Returns HvacMode enum value from the string value.
    /// Accepted values: ["OFF", "HEAT", "COOL", "HEATCOOL"]
    pub fn from_string(string: String) -> Result<Self, HvacModeConversionError> {
        match string.to_uppercase().as_str() {
            "OFF" => Ok(HvacMode::Off),
            "HEAT" => Ok(HvacMode::Heat),
            "COOL" => Ok(HvacMode::Cool),
            "HEATCOOL" => Ok(HvacMode::HeatCool),
            _ => Err(HvacModeConversionError::UnknownValue(string)),
        }
    }
}

impl HvacStatus {
    /// Returns HvacStatus enum value from the string value.
    pub fn from_string(string: String) -> Result<Self, HvacStatusConversionError> {
        match string.to_uppercase().as_str() {
            "OFF" => Ok(HvacStatus::Off),
            "IDLE" => Ok(HvacStatus::Idle),
            "COOLING" => Ok(HvacStatus::Cooling),
            "HEAT_PUMP" => Ok(HvacStatus::HeatPump),
            "ELEC_FURNACE" => Ok(HvacStatus::ElectricFurnace),
            "GAS_FURNACE" => Ok(HvacStatus::GasFurnace),
            "GAS_OVERRIDE" => Ok(HvacStatus::GasOverride),
            "DEAD_TIME" => Ok(HvacStatus::DeadTime),
            "LOAD_SHEDDING" => Ok(HvacStatus::LoadShedding),
            "FAIL_OFF" => Ok(HvacStatus::FailOff),
            "FAIL_IDLE" => Ok(HvacStatus::FailIdle),
            "FAIL_COOLING" => Ok(HvacStatus::FailCooling),
            "FAIL_HEAT_PUMP" => Ok(HvacStatus::FailHeatPump),
            "FAIL_ELEC_FURNACE" => Ok(HvacStatus::FailElectricFurnace),
            "FAIL_GAS_FURNACE" => Ok(HvacStatus::FailGasFurnace),
            "FAIL_GAS_OVERRIDE" => Ok(HvacStatus::FailGasOverride),
            "FAIL_DEAD_TIME" => Ok(HvacStatus::FailDeadTime),
            "FAIL_SHEDDING" => Ok(HvacStatus::FailShedding),
            _ => Err(HvacStatusConversionError::UnknownValue(string)),
        }
    }
}

impl HVAC {
    /// Create a new HVAC from a generic "Thing". LCI gateway must publish the type as a HVAC.
    pub fn new(thing: Thing) -> Result<Self, HvacError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Hvac) {
            Ok(Self { thing })
        } else {
            Err(HvacError::InvalidDeviceType(thing_type))
        }
    }

    /// Returns the label of the device such as "Bedroom HVAC".
    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    /// Gets the current HVAC status
    pub async fn status(&self) -> Result<HvacStatus, HvacStatusConversionError> {
        let string = common::get_field(&self.thing, "status").await?;
        HvacStatus::from_string(string)
    }

    /// Get the "outside temperature". Accuracy seems questionable.
    pub async fn outside_temperature(&self) -> Result<f32, HvacOutsideTemperatureFailure> {
        let string = common::get_field(&self.thing, "outside_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacOutsideTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    /// Get the temperature inside the room.
    pub async fn inside_temperature(&self) -> Result<f32, HvacInsideTemperatureFailure> {
        let string = common::get_field(&self.thing, "inside_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacInsideTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    /// Get the temperature for when the A/C should start to cool.
    pub async fn high_temperature(&self) -> Result<f32, HvacHighTemperatureFailure> {
        let string = common::get_field(&self.thing, "high_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacHighTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    /// Get the temperature for when the A/C should start to heat.
    pub async fn low_temperature(&self) -> Result<f32, HvacLowTemperatureFailure> {
        let string = common::get_field(&self.thing, "low_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacLowTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    /// Get the current fan mode.
    pub async fn fan(&self) -> Result<HvacFanMode, HvacFanModeConversionError> {
        let string = common::get_field(&self.thing, "fan_mode").await?;
        HvacFanMode::from_string(string)
    }

    /// Get the current HvacMode.
    pub async fn mode(&self) -> Result<HvacMode, HvacModeConversionError> {
        let string = common::get_field(&self.thing, "hvac_mode").await?;
        HvacMode::from_string(string)
    }

    /// Set the temperature for which the A/C should start cooling.
    pub async fn set_high_temperature(&mut self, temp: isize) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "high_temperature", temp.to_string()).await?;
        Ok(())
    }

    /// Set the temperature for which the A/C should start heating.
    pub async fn set_low_temperature(&mut self, temp: isize) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "low_temperature", temp.to_string()).await?;
        Ok(())
    }

    /// Set the fan mode.
    pub async fn set_fan(&mut self, mode: &HvacFanMode) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "fan_mode", mode.to_string().to_uppercase()).await?;
        Ok(())
    }

    /// Set the HVAC mode.
    pub async fn set_mode(&mut self, mode: &HvacMode) -> Result<(), common::SetError> {
        common::set_field(
            &mut self.thing,
            "hvac_mode",
            mode.to_string().to_uppercase(),
        )
        .await?;
        Ok(())
    }
}

impl std::fmt::Display for HvacFanMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fan = match self {
            HvacFanMode::Auto => "Auto".to_string(),
            HvacFanMode::Low => "Low".to_string(),
            HvacFanMode::High => "High".to_string(),
        };
        write!(f, "{}", fan)
    }
}

impl std::fmt::Display for HvacMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            HvacMode::Off => "Off".to_string(),
            HvacMode::Heat => "Heat".to_string(),
            HvacMode::Cool => "Cool".to_string(),
            HvacMode::HeatCool => "HeatCool".to_string(),
        };
        write!(f, "{}", mode)
    }
}

impl std::fmt::Display for HvacStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            HvacStatus::Off => "Off".to_string(),
            HvacStatus::Idle => "Idle".to_string(),
            HvacStatus::Cooling => "Cooling".to_string(),
            HvacStatus::HeatPump => "Heat Pump".to_string(),
            HvacStatus::ElectricFurnace => "Electric Furnace".to_string(),
            HvacStatus::GasFurnace => "Gas Furnace".to_string(),
            HvacStatus::GasOverride => "Gas Override".to_string(),
            HvacStatus::DeadTime => "Dead Time".to_string(),
            HvacStatus::LoadShedding => "Load Shedding".to_string(),
            HvacStatus::FailOff => "Load Off".to_string(),
            HvacStatus::FailIdle => "Load Idle".to_string(),
            HvacStatus::FailCooling => "Fail Cooling".to_string(),
            HvacStatus::FailHeatPump => "Fail Heat Pump".to_string(),
            HvacStatus::FailElectricFurnace => "Fail Electric Furnace".to_string(),
            HvacStatus::FailGasFurnace => "Fail Gas Furnace".to_string(),
            HvacStatus::FailGasOverride => "Fail Gas Override".to_string(),
            HvacStatus::FailDeadTime => "Fail Dead Time".to_string(),
            HvacStatus::FailShedding => "Fail Shedding".to_string(),
        };
        write!(f, "{}", mode)
    }
}

/// Returned when a HVAC can not be made from the given "thing".
#[derive(Debug, Error)]
pub enum HvacError {
    /// Returned when the "thing" is not a Hvac.
    #[error("The device type {0:?} is not a Hvac.")]
    InvalidDeviceType(Option<DeviceType>),
}

/// The HVAC fan state could not be fetched.
#[derive(Debug, Error)]
pub enum HvacFanModeConversionError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{0}' could not be converted to a HvacFanMode.")]
    UnknownValue(String),
}

/// The HVAC mode could not be fetched.
#[derive(Debug, Error)]
pub enum HvacModeConversionError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{0}' could not be converted to a HvacMode.")]
    UnknownValue(String),
}

/// The HVAC status could not be fetched.
#[derive(Debug, Error)]
pub enum HvacStatusConversionError {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be understood.
    #[error("The given value '{0}' could not be converted to a HvacStatus.")]
    UnknownValue(String),
}

/// The HVAC outside temperature could not be fetched.
#[derive(Debug, Error)]
pub enum HvacOutsideTemperatureFailure {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseFloatError),
}

/// The HVAC inside temperature could not be fetched.
#[derive(Debug, Error)]
pub enum HvacInsideTemperatureFailure {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseFloatError),
}

/// The HVAC status could not be fetched.
#[derive(Debug, Error)]
pub enum HvacStatusFailure {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseFloatError),
}

/// The HVAC high temperature could not be fetched.
#[derive(Debug, Error)]
pub enum HvacHighTemperatureFailure {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseFloatError),
}

/// The HVAC low temperature could not be fetched.
#[derive(Debug, Error)]
pub enum HvacLowTemperatureFailure {
    /// The request to the LCI gateway failed.
    #[error("The LCI gateway could not be reached. {0}")]
    GetFailure(common::GetFailure),
    /// The response from the LCI gateway could not be parsed.
    #[error("The response from the LCI gateway could not be parsed, {1}")]
    Parse(String, std::num::ParseFloatError),
}

impl From<common::GetFailure> for HvacOutsideTemperatureFailure {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacInsideTemperatureFailure {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacStatusConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacHighTemperatureFailure {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacLowTemperatureFailure {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacFanModeConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacModeConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}
