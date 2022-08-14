#![allow(non_snake_case)]

use derive_getters::Getters;
use serde::Deserialize;

#[derive(PartialEq)]
pub enum DeviceType {
    Gateway,
    Tank,
    RgbLights,
    Hvac,
    Dimmer,
    Generator,
    Switch,
}

#[derive(Getters, Deserialize, Debug)]
pub struct Configuration {
    deviceType: Option<f32>,
    capability: Option<f32>,
}

#[derive(Getters, Deserialize, Debug)]
pub struct StatusInfo {
    status: String,
    statusDetails: String,
}

#[derive(Getters, Deserialize, Debug)]
pub struct Channel {
    uid: String,
    id: String,
    channelTypeUID: String,
    itemType: String,
}

#[derive(Getters, Deserialize, Debug)]
pub struct Thing {
    UID: String,
    label: String,
    configuration: Configuration, // TODO: can't figure out how to make this optional
    channels: Vec<Channel>,
}

#[derive(Getters, Deserialize, Debug)]
pub struct LinkState {
    link: String,
    state: String,
}

impl Thing {
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
}
