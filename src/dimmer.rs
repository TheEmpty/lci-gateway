use super::{common, DeviceType, Thing};

pub struct Dimmer {
    thing: Thing,
}

impl Dimmer {
    pub fn new(thing: Thing) -> Result<Self, DimmerError> {
        let thing_type = thing.get_type();
        if thing_type == Some(DeviceType::Dimmer) {
            Ok(Self { thing })
        } else {
            Err(DimmerError::InvalidDeviceType(thing_type))
        }
    }

    pub fn label(&self) -> String {
        self.thing.label().clone()
    }

    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(&self.thing).await
    }

    pub async fn brightness(&self) -> Result<common::Percentage, DimmerBrightnessError> {
        let string = common::get_field(&self.thing, "dimmer")
            .await
            .map_err(DimmerBrightnessError::GetFailure)?;
        let val = string
            .parse::<u8>()
            .map_err(|e| DimmerBrightnessError::Parse(string, e))?;
        Ok(common::Percentage::new(val))
    }

    pub async fn on(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "dimmer", "ON".to_string()).await?;
        Ok(())
    }

    pub async fn off(&mut self) -> Result<(), common::SetError> {
        common::set_field(&mut self.thing, "dimmer", "OFF".to_string()).await?;
        Ok(())
    }

    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), SetBrightnessError> {
        if brightness > 100 {
            return Err(SetBrightnessError::InvalidValue(brightness));
        }
        common::set_field(&mut self.thing, "dimmer", brightness.to_string()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum DimmerError {
    InvalidDeviceType(Option<DeviceType>),
}

#[derive(Debug)]
pub enum DimmerBrightnessError {
    GetFailure(common::GetFailure),
    Parse(String, std::num::ParseIntError),
}

#[derive(Debug)]
pub enum SetBrightnessError {
    InvalidValue(u8),
    SetError(common::SetError),
}

impl From<common::SetError> for SetBrightnessError {
    fn from(error: common::SetError) -> Self {
        Self::SetError(error)
    }
}
