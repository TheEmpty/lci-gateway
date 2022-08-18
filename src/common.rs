use super::{LinkState, Thing};

#[derive(Debug, PartialEq, Eq)]
pub enum OnlineState {
    Offline,
    Online,
    Locked,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Percentage {
    value: u8,
}

pub async fn get_field(thing: &Thing, field: &str) -> Result<String, GetFailure> {
    let item_id = thing.UID().replace(':', "_").replace('-', "_");
    let url = format!("http://192.168.1.4:8080/rest/items/{}_{}", item_id, field);
    let res = reqwest::get(&url).await.map_err(GetFailure::Request)?;
    let txt = res.text().await.map_err(GetFailure::Text)?;
    let state: LinkState = serde_json::from_str(&txt).map_err(GetFailure::SerdeJsonConversion)?;
    Ok(state.state().to_string())
}

pub async fn set_field(
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
    pub fn from_string(string: String) -> Result<Self, OnlineStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(OnlineState::Offline),
            "ON" => Ok(OnlineState::Online),
            "LOCKED" => Ok(OnlineState::Locked),
            _ => Err(OnlineStateConversionError::UnknownValue(string)),
        }
    }
}

impl Percentage {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

pub async fn get_online_state(thing: &Thing) -> Result<OnlineState, OnlineStateConversionError> {
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

#[derive(Debug)]
pub enum GetFailure {
    Request(reqwest::Error),
    Text(reqwest::Error),
    SerdeJsonConversion(serde_json::Error),
}

#[derive(Debug)]
pub enum OnlineStateConversionError {
    UnknownValue(String),
    GetFailure(GetFailure),
}

#[derive(Debug)]
pub enum SetError {
    Send(reqwest::Error),
    Status(u16, reqwest::Response),
}
