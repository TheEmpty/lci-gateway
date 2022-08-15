use super::{common, DeviceType, Thing};

pub struct HVAC {
    thing: Thing,
}

#[derive(Debug)]
pub enum HvacFanConversionError {
    UnknownValue(String),
}

pub enum HvacFan {
    Auto,
    Low,
    High,
}

impl HvacFan {
    pub fn to_string(&self) -> String {
        match self {
            HvacFan::Auto => "AUTO".to_string(),
            HvacFan::Low => "LOW".to_string(),
            HvacFan::High => "HIGH".to_string(),
        }
    }

    pub fn from_string(string: String) -> Result<Self, HvacFanConversionError> {
        match string.as_str() {
            "AUTO" => Ok(HvacFan::Auto),
            "LOW" => Ok(HvacFan::Low),
            "HIGH" => Ok(HvacFan::High),
            _ => Err(HvacFanConversionError::UnknownValue(string)),
        }
    }
}

impl std::fmt::Display for HvacFan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub enum HvacModeConversionError {
    UnknownValue(String),
}

pub enum HvacMode {
    Off,
    Heat,
    Cool,
    HeatCool,
}

impl HvacMode {
    pub fn to_string(&self) -> String {
        match self {
            HvacMode::Off => "OFF".to_string(),
            HvacMode::Heat => "HEAT".to_string(),
            HvacMode::Cool => "COOL".to_string(),
            HvacMode::HeatCool => "HEATCOOL".to_string(),
        }
    }

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

impl std::fmt::Display for HvacMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl HVAC {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Hvac));
        Ok(Self { thing })
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn outside_temprature(&self) -> String {
        common::get_field(&self.thing, "outside_temperature").await
    }

    pub async fn inside_temprature(&self) -> String {
        common::get_field(&self.thing, "inside_temperature").await
    }

    pub async fn status(&self) -> String {
        common::get_field(&self.thing, "status").await
    }

    pub async fn high_temp(&self) -> String {
        common::get_field(&self.thing, "high_temperature").await
    }

    pub async fn set_high_temp(&mut self, temp: isize) {
        // TODO: use results
        let _ = common::set_field(&mut self.thing, "high_temperature", temp.to_string()).await;
    }

    pub async fn low_temp(&self) -> String {
        common::get_field(&self.thing, "low_temperature").await
    }

    pub async fn set_low_temp(&mut self, temp: isize) {
        // TODO: use results
        let _ = common::set_field(&mut self.thing, "low_temperature", temp.to_string()).await;
    }

    pub async fn fan(&self) -> Result<HvacFan, HvacFanConversionError> {
        let string = common::get_field(&self.thing, "fan_mode").await;
        HvacFan::from_string(string)
    }

    pub async fn set_fan(&mut self, mode: &HvacFan) {
        // TODO: use results
        let _ = common::set_field(&mut self.thing, "fan_mode", mode.to_string()).await;
    }

    pub async fn mode(&self) -> Result<HvacMode, HvacModeConversionError> {
        let string = common::get_field(&self.thing, "hvac_mode").await;
        HvacMode::from_string(string)
    }

    pub async fn set_mode(&mut self, mode: &HvacMode) {
        // TODO: use results
        let _ = common::set_field(&mut self.thing, "hvac_mode", mode.to_string()).await;
    }
}
