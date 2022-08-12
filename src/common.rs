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
    thing: &Thing,
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
