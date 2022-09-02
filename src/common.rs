use super::Thing;
use derive_getters::Getters;
use serde::Deserialize;
use thiserror::Error;

/// A device's online state in the router. Offline meaning it can not be communicated with.
#[derive(Debug, PartialEq, Eq)]
pub enum OnlineState {
    /// Device is not responsive.
    Offline,
    /// Device is responsive and functional.
    Online,
    /// Unknown use-case.
    Locked,
}

/// A 0-100 percentage.
#[derive(Debug, PartialEq, Eq)]
pub struct Percentage {
    value: u8,
}

#[derive(Getters, Deserialize, Debug)]
pub(crate) struct LinkState {
    state: String,
}

pub(crate) async fn get_field(thing: &Thing, field: &str) -> Result<String, GetFailure> {
    let item_id = thing.UID().replace(':', "_").replace('-', "_");
    let url = format!("http://192.168.1.4:8080/rest/items/{}_{}", item_id, field);
    let res = reqwest::get(&url).await.map_err(GetFailure::Request)?;
    let txt = res.text().await.map_err(GetFailure::Text)?;
    let state: LinkState = serde_json::from_str(&txt).map_err(GetFailure::SerdeJsonConversion)?;
    Ok(state.state().to_string())
}

pub(crate) async fn set_field(
    thing: &mut Thing,
    field: &str,
    value: String,
) -> Result<reqwest::Response, SetError> {
    let item_id = thing.UID().replace(':', "_").replace('-', "_");
    let url = format!("http://192.168.1.4:8080/rest/items/{}_{}", item_id, field);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .body(value)
        .send()
        .await
        .map_err(SetError::Send)?;

    if response.status().is_success() {
        Ok(response)
    } else {
        Err(SetError::Status(response.status().as_u16(), response))
    }
}

impl OnlineState {
    /// Returns an online state enum value from the string value.
    /// Accepted values: ["OFF", "ON", "LOCKED"]
    pub fn from_string(string: String) -> Result<Self, OnlineStateConversionError> {
        match string.to_uppercase().as_str() {
            "OFF" => Ok(OnlineState::Offline),
            "ON" => Ok(OnlineState::Online),
            "LOCKED" => Ok(OnlineState::Locked),
            _ => Err(OnlineStateConversionError::UnknownValue(string)),
        }
    }
}

impl Percentage {
    pub(crate) fn new(value: u8) -> Self {
        Self { value }
    }

    /// Gets the 0-100 value of the percentage.
    pub fn value(&self) -> u8 {
        self.value
    }
}

pub(crate) async fn get_online_state(
    thing: &Thing,
) -> Result<OnlineState, OnlineStateConversionError> {
    let string = get_field(thing, "online")
        .await
        .map_err(OnlineStateConversionError::GetFailure)?;
    OnlineState::from_string(string)
}

impl std::fmt::Display for OnlineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            OnlineState::Online => "Online".to_string(),
            OnlineState::Offline => "Offline".to_string(),
            OnlineState::Locked => "Locked".to_string(),
        };
        write!(f, "{}", result)
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.value())
    }
}

/// When calling the LCI gateway can not be reached or returns bad data.
#[derive(Debug, Error)]
pub enum GetFailure {
    /// The LCI gateway could not be reached, see the inner request error.
    #[error("The LCI gateway could not be reached. {0}")]
    Request(reqwest::Error),
    /// Could not get the text from the HTTP response.
    #[error("The text could not be retrieved. {0}")]
    Text(reqwest::Error),
    /// The LCI gateway returned unexpected or invalid JSON.
    #[error("The JSON response could not be parsed. {0}")]
    SerdeJsonConversion(serde_json::Error),
}

/// Failed to get the online state of the device.
#[derive(Debug, Error)]
pub enum OnlineStateConversionError {
    /// The provided value could not be mapped to the OnlineState enum.
    #[error("The given value '{0}' could not be converted to an OnlineState.")]
    UnknownValue(String),
    /// The request to the LCI gateway failed.
    #[error("The request to the LCI gateway failed. {0}")]
    GetFailure(GetFailure),
}

/// Failed to set the value in the LCI gateway.
#[derive(Debug, Error)]
pub enum SetError {
    /// The request failed to send to the gateway.
    #[error("The request failed to send to the gateway. {0}")]
    Send(reqwest::Error),
    /// The response from the gateway did not indicate acceptance.
    #[error("The http response code from the gateway, {0}, did not indicate success.")]
    Status(u16, reqwest::Response),
}
