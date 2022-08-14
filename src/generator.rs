use super::{common, DeviceType, Thing};

pub struct Generator {
    thing: Thing,
}

impl Generator {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Generator));
        Ok(Self { thing })
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn on(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "command", "ON".to_string()).await;
    }

    pub async fn off(&self) {
        // TODO: care about the result.
        let _ = common::set_field(&self.thing, "command", "OFF".to_string()).await;
    }

    pub async fn state(&self) -> String {
        common::get_field(&self.thing, "command").await
    }
}
