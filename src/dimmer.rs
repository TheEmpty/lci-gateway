use super::{common, DeviceType, Thing};

pub struct Dimmer {
    thing: Thing,
}

impl Dimmer {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Dimmer));
        Ok(Self { thing })
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub async fn brightness(&self) -> String {
        common::get_field(&self.thing, "dimmer").await
    }

    pub async fn on(&mut self) {
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "dimmer", "ON".to_string()).await;
    }

    pub async fn off(&mut self) {
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "dimmer", "OFF".to_string()).await;
    }

    pub async fn set_brightness(&mut self, brightness: usize) {
        assert!(brightness <= 100);
        // TODO: care about the result.
        let _ = common::set_field(&mut self.thing, "dimmer", brightness.to_string()).await;
    }
}
