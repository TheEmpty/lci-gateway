use super::{Channel, Thing};

pub struct Dimmer {
    url: String,
}

impl Dimmer {
    pub fn new(thing: Thing) -> Result<Self, ()> {
        let channels: Vec<&Channel> = thing
            .channels()
            .iter()
            .filter(|x| x.id() == "dimmer")
            .collect();
        if channels.len() == 1 {
            let channel = channels.get(0).expect("Failed to find dimmer");
            let item_id = channel.uid().replace(":", "_").replace("-", "_");
            let url = format!("http://192.168.1.4:8080/rest/items/{}", item_id);
            Ok(Self { url })
        } else {
            Err(())
        }
    }

    pub async fn on(&self) {
        let client = reqwest::Client::new();
        let _res = client
            .post(&self.url)
            .header("Accept", "application/json")
            .body("ON")
            .send()
            .await;
    }

    pub async fn off(&self) {
        let client = reqwest::Client::new();
        let _res = client
            .post(&self.url)
            .header("Accept", "application/json")
            .body("OFF")
            .send()
            .await;
    }

    pub async fn brightness(&self, brightness: usize) {
        assert!(brightness <= 100);
        let client = reqwest::Client::new();
        let _res = client
            .post(&self.url)
            .header("Accept", "application/json")
            .body(format!("{}", brightness))
            .send()
            .await;
    }
}
