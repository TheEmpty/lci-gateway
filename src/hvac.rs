use super::{common, DeviceType, Thing};

pub struct HVAC {
    thing: Thing,
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
}

impl HVAC {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Hvac));
        Ok(Self { thing })
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

    pub async fn set_high_temp(&self, temp: isize) {
        // TODO: use results
        let _ = common::set_field(&self.thing, "high_temperature", temp.to_string()).await;
    }

    pub async fn low_temp(&self) -> String {
        common::get_field(&self.thing, "low_temperature").await
    }

    pub async fn set_low_temp(&self, temp: isize) {
        // TODO: use results
        let _ = common::set_field(&self.thing, "low_temperature", temp.to_string()).await;
    }

    pub async fn fan(&self) -> String {
        common::get_field(&self.thing, "fan_mode").await
    }

    pub async fn set_fan(&self, mode: &HvacFan) {
        // TODO: use results
        let _ = common::set_field(&self.thing, "fan_mode", mode.to_string()).await;
    }

    pub async fn mode(&self) -> String {
        common::get_field(&self.thing, "hvac_mode").await
    }

    pub async fn set_mode(&self, mode: &HvacMode) {
        // TODO: use results
        let _ = common::set_field(&self.thing, "hvac_mode", mode.to_string()).await;
    }
}
