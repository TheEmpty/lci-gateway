mod dimmer;
mod switch;
mod sse;
mod generator;

pub use dimmer::Dimmer;
pub use switch::Switch;
pub use sse::Channel;
pub use sse::Configuration;
pub use sse::StatusInfo;
pub use sse::Thing;
pub use generator::Generator;

#[derive(Debug)]
pub enum ThingError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

pub async fn get_things() -> Result<Vec<Thing>, ThingError> {
    log::trace!("Fetching things");
    let body = reqwest::get("http://192.168.1.4:8080/rest/things/")
        .await
        .map_err(ThingError::Reqwest)?
        .text()
        .await
        .map_err(ThingError::Reqwest)?;
    let things: Vec<Thing> = serde_json::from_str(&body).map_err(ThingError::Json)?;
    Ok(things)
}