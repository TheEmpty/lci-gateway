#![allow(non_snake_case)]

use derive_getters::Getters;
use serde::Deserialize;

/*
deviceType = None = Gateway
deviceType = 10 = tank (grey, white, fuel, etc)
deivceType = 13 = RGB Lights "color"
deviceType = 16 = HVAC
deviceType = 24 = generator
deivecType = 30 = On/off switch
*/

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
