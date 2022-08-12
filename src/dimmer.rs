use super::{common, Thing};

pub struct Dimmer {
    thing: Thing,
}

impl Dimmer {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        // TODO: confirm DeviceType
        Ok(Self { thing })
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn brightness(&self) -> String {
        common::get_field(&self.thing, "dimmer").await
    }

    pub async fn on(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "dimmer", "ON".to_string()).await;
    }

    pub async fn off(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "dimmer", "OFF".to_string()).await;
    }

    pub async fn set_brightness(&self, brightness: usize) {
        assert!(brightness <= 100);
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "dimmer", brightness.to_string()).await;
    }
}
