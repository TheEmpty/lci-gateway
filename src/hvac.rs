use super::{common, DeviceType, Thing};

pub struct HVAC {
    thing: Thing,
}

#[derive(Debug)]
pub enum HvacFan {
    Auto,
    Low,
    High,
}

#[derive(Debug)]
pub enum HvacMode {
    Off,
    Heat,
    Cool,
    HeatCool,
}

impl HvacFan {
    pub fn from_string(string: String) -> Result<Self, HvacFanConversionError> {
        match string.as_str() {
            "AUTO" => Ok(HvacFan::Auto),
            "LOW" => Ok(HvacFan::Low),
            "HIGH" => Ok(HvacFan::High),
            _ => Err(HvacFanConversionError::UnknownValue(string)),
        }
    }
}

impl HvacMode {
    pub fn from_string(string: String) -> Result<Self, HvacModeConversionError> {
        match string.as_str() {
            "OFF" => Ok(HvacMode::Off),
            "HEAT" => Ok(HvacMode::Heat),
            "COOL" => Ok(HvacMode::Cool),
            "HEATCOOL" => Ok(HvacMode::HeatCool),
            _ => Err(HvacModeConversionError::UnknownValue(string)),
        }
    }
}

impl HVAC {
    pub fn new(thing: Thing) -> Result<Self, HvacError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Hvac) {
            Ok(Self { thing })
        } else {
            Err(HvacError::InvalidDeviceType(thing_type))
        }
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn status(&self) -> Result<String, HvacStatusFailure> {
        Ok(common::get_field(&self.thing, "status").await?)
    }

    pub async fn outside_temprature(&self) -> Result<f32, HvacOutsideTemperatureFailure> {
        let string = common::get_field(&self.thing, "outside_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacOutsideTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    pub async fn inside_temprature(&self) -> Result<f32, HvacInsideTemperatureFailure> {
        let string = common::get_field(&self.thing, "inside_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacInsideTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    pub async fn high_temp(&self) -> Result<f32, HvacHighTemperatureFailure> {
        let string = common::get_field(&self.thing, "high_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacHighTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    pub async fn low_temp(&self) -> Result<f32, HvacLowTemperatureFailure> {
        let string = common::get_field(&self.thing, "low_temperature").await?;
        let val = string
            .parse::<f32>()
            .map_err(|e| HvacLowTemperatureFailure::Parse(string, e))?;
        Ok(val)
    }

    pub async fn fan(&self) -> Result<HvacFan, HvacFanConversionError> {
        let string = common::get_field(&self.thing, "fan_mode").await?;
        HvacFan::from_string(string)
    }

    pub async fn mode(&self) -> Result<HvacMode, HvacModeConversionError> {
        let string = common::get_field(&self.thing, "hvac_mode").await?;
        HvacMode::from_string(string)
    }

    pub async fn set_high_temp(&mut self, temp: isize) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "high_temperature", temp.to_string()).await?;
        Ok(())
    }

    pub async fn set_low_temp(&mut self, temp: isize) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "low_temperature", temp.to_string()).await?;
        Ok(())
    }

    pub async fn set_fan(&mut self, mode: &HvacFan) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "fan_mode", mode.to_string().to_uppercase()).await?;
        Ok(())
    }

    pub async fn set_mode(&mut self, mode: &HvacMode) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "hvac_mode", mode.to_string().to_uppercase()).await?;
        Ok(())
    }
}

impl std::fmt::Display for HvacFan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fan = match self {
            HvacFan::Auto => "Auto".to_string(),
            HvacFan::Low => "Low".to_string(),
            HvacFan::High => "High".to_string(),
        };
        write!(f, "{}", fan)
    }
}

impl std::fmt::Display for HvacMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode =  match self {
            HvacMode::Off => "Off".to_string(),
            HvacMode::Heat => "Heat".to_string(),
            HvacMode::Cool => "Cool".to_string(),
            HvacMode::HeatCool => "HeatCool".to_string(),
        };
        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub enum HvacError {
    InvalidDeviceType(Option<DeviceType>),
}

#[derive(Debug)]
pub enum HvacFanConversionError {
    UnknownValue(String),
    GetFailure(common::GetFailure),
}

#[derive(Debug)]
pub enum HvacModeConversionError {
    UnknownValue(String),
    GetFailure(common::GetFailure),
}

#[derive(Debug)]
pub enum HvacOutsideTemperatureFailure {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseFloatError),
}

#[derive(Debug)]
pub enum HvacInsideTemperatureFailure {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseFloatError),
}

#[derive(Debug)]
pub enum HvacStatusFailure {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseFloatError),
}

#[derive(Debug)]
pub enum HvacHighTemperatureFailure {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseFloatError),
}

#[derive(Debug)]
pub enum HvacLowTemperatureFailure {
    GetFailure(common::GetFailure),
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

impl From<common::GetFailure> for HvacStatusFailure {
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

impl From<common::GetFailure> for HvacFanConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}

impl From<common::GetFailure> for HvacModeConversionError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}
