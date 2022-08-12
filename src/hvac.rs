use super::{common, Thing};

pub struct HVAC {
    thing: Thing,
}

// hvac_mode
// http://192.168.1.4:8080/rest/items/idsmyrv_hvac_thing_0000001227E31001_hvac_mode
// fan mode
// http://192.168.1.4:8080/rest/items/idsmyrv_hvac_thing_0000001227E31001_fan_mode
// high temp
// http://192.168.1.4:8080/rest/items/idsmyrv_hvac_thing_0000001227E31001_high_temperature
// low temp
// http://192.168.1.4:8080/rest/items/idsmyrv_hvac_thing_0000001227E31001_low_temperature

impl HVAC {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        // TODO: confirm DeviceType
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
}
