#![allow(non_snake_case)]

use super::common;
use derive_getters::Getters;
use serde::Deserialize;

/// The type of device this "thing" represents.
#[derive(Debug, PartialEq, Eq)]
pub enum DeviceType {
    /// This is the root gateway. Not supported.
    Gateway,
    /// A tank that holds things.
    Tank,
    /// RGB Lights. Not supported.
    RgbLights,
    /// HVAC aka A/C
    Hvac,
    /// Dimmer lights.
    Dimmer,
    /// Generators
    Generator,
    /// Switches that can only be on or off.
    Switch,
}

/// How the device is configured to work in LCI.
#[derive(Getters, Deserialize, Debug)]
pub struct Configuration {
    /// The device type as an integer.
    /// Not set for LCI Gateway thing.
    deviceType: Option<f32>,
}

/// The "thing" in the LCI system.
#[derive(Getters, Deserialize, Debug)]
pub struct Thing {
    /// The name of this thing.
    label: String,
    /// The given UID- unique ID.
    UID: String,
    /// What type of device this is configured as.
    configuration: Configuration, // TODO: can't figure out how to make this optional
}

impl Thing {
    /// Returns the type of device this "thing" is configured as.
    pub fn get_type(&self) -> Option<DeviceType> {
        match self.configuration().deviceType() {
            None => Some(DeviceType::Gateway),
            x if x == &Some(10.0) => Some(DeviceType::Tank),
            x if x == &Some(13.0) => Some(DeviceType::RgbLights),
            x if x == &Some(16.0) => Some(DeviceType::Hvac),
            x if x == &Some(20.0) => Some(DeviceType::Dimmer),
            x if x == &Some(24.0) => Some(DeviceType::Generator),
            x if x == &Some(30.0) => Some(DeviceType::Switch),
            _ => None,
        }
    }

    /// Returns the device's online state.
    pub async fn online(&self) -> Result<common::OnlineState, common::OnlineStateConversionError> {
        common::get_online_state(self).await
    }
}
