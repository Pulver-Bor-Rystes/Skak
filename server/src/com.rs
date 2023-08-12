use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

use crate::server::SocketContext;

/// har to felter: topic og data
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WSMessage<Data = String> {
    pub topic: String,
    pub data: Data,
}

// Vi har basically to identiske data typer. Dog er den ene ment til at være en fejl besked
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Success<Data = String> {
    result: bool,
    data: Data,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Failure<Data = String> {
    result: bool,
    error: Data,
}

impl Failure {
    pub fn new<T: Serialize>(error: T) -> Failure<T> {
        Failure {
            result: false,
            error,
        }
    }
}

impl Success {
    pub fn new<T: Serialize>(data: T) -> Success<T> {
        Success { result: true, data }
    }
}

impl WSMessage {
    pub fn ping() -> Self {
        WSMessage {
            topic: "ping".to_string(),
            data: "ping message".to_string(),
        }
    }

    pub fn something_went_wrong(data: String) -> WSMessage<Failure<String>> {
        let parsed: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);

        let mut topic = "no topic".to_string();

        if let Ok(parsed) = parsed {
            if let Some(parsed_topic) = parsed["topic"].as_str() {
                topic = parsed_topic.to_owned();
            }
        }

        WSMessage {
            topic: topic.to_string(),
            data: Failure::new(data),
        }
    }
}
