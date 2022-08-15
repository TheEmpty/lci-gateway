use super::{common, DeviceType, Thing};

pub struct Tank {
    thing: Thing,
}

impl Tank {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        assert!(thing.get_type() == Some(DeviceType::Tank));
        Ok(Self { thing })
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn level(&self) -> String {
        common::get_field(&self.thing, "tank_level").await
    }
}
