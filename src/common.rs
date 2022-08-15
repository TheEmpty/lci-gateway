use super::{LinkState, Thing};

pub async fn get_field(thing: &Thing, field: &str) -> String {
    let item_id = thing.UID().replace(":", "_").replace("-", "_");
    let url = format!("http://192.168.1.4:8080/rest/items/{}_{}", item_id, field);
    let res = reqwest::get(&url).await.expect("Failed to get field");
    let txt = res.text().await.expect("Failed to get text");
    let state: LinkState = serde_json::from_str(&txt).expect("Failed to deserialize");
    state.state().to_string()
}

pub async fn set_field(
    thing: &mut Thing,
    field: &str,
    value: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let item_id = thing.UID().replace(":", "_").replace("-", "_");
    let url = format!("http://192.168.1.4:8080/rest/items/{}_{}", item_id, field);
    let client = reqwest::Client::new();
    client
        .post(&url)
        .header("Accept", "application/json")
        .body(value)
        .send()
        .await
}

pub enum OnlineState {
    Online,
    Offline,
    Locked,
}

impl OnlineState {
    pub fn to_string(&self) -> String {
        match self {
            OnlineState::Online => "Online".to_string(),
            OnlineState::Offline => "Offline".to_string(),
            OnlineState::Locked => "Locked".to_string(),
        }
    }

    pub fn from_string(string: String) -> Result<Self, OnlineStateConversionError> {
        match string.as_str() {
            "OFF" => Ok(OnlineState::Offline),
            "ON" => Ok(OnlineState::Online),
            "LOCKED" => Ok(OnlineState::Locked),
            _ => Err(OnlineStateConversionError::UnknownValue(string)),
        }
    }
}

#[derive(Debug)]
pub enum OnlineStateConversionError {
    UnknownValue(String),
}

pub async fn get_online_state(thing: &Thing) -> Result<OnlineState, OnlineStateConversionError> {
    let string = get_field(&thing, "online").await;
    OnlineState::from_string(string)
}

impl std::fmt::Display for OnlineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
