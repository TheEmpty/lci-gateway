use super::{common, Thing};

pub struct Tank {
    thing: Thing,
}

impl Tank {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        // TODO: confirm DeviceType
        Ok(Self { thing })
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn level(&self) -> String {
        common::get_field(&self.thing, "tank_level").await
    }
}
