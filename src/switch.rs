use super::{common, DeviceType, Thing};

pub struct Switch {
    thing: Thing,
}

impl Switch {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Switch));
        Ok(Self { thing })
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn on(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "switch", "ON".to_string()).await;
    }

    pub async fn off(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "switch", "OFF".to_string()).await;
    }

    pub async fn state(&self) -> String {
        common::get_field(&self.thing, "switch").await
    }
}
