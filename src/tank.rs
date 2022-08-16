use super::{common, DeviceType, Thing};

pub struct Tank {
    thing: Thing,
}

impl Tank {
    pub fn new(thing: Thing) -> Result<Self, TankError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Tank) {
            Ok(Self { thing })
        } else {
            Err(TankError::InvalidDeviceType(thing_type))
        }
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn level(&self) -> Result<common::Percentage, TankLevelError> {
        let string = common::get_field(&self.thing, "tank_level").await?;
        let val = string
            .parse::<u8>()
            .map_err(|e| TankLevelError::Parse(string, e))?;
        Ok(common::Percentage::new(val))
    }
}

#[derive(Debug)]
pub enum TankError {
    InvalidDeviceType(Option<DeviceType>),
}

#[derive(Debug)]
pub enum TankLevelError {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseIntError),
}

impl From<common::GetFailure> for TankLevelError {
    fn from(error: common::GetFailure) -> Self {
        Self::GetFailure(error)
    }
}
