use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

use crate::server::MyWebSocket;

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

pub struct Context<'a, CTX> {
    ctx: &'a mut CTX,
    sent: bool,
}

// En trait så vi kan videre give en context til en handle funktion
pub trait MessageHandler {
    fn send(&mut self, message: impl Serialize);

    fn ok(&mut self, topic: impl ToString, data: impl Serialize) {
        self.send(WSMessage {
            topic: topic.to_string(),
            data: Success { result: true, data },
        })
    }

    fn error(&mut self, topic: impl ToString, error: impl Serialize) {
        self.send(WSMessage {
            topic: topic.to_string(),
            data: Failure {
                result: false,
                error,
            },
        })
    }
}

// Hjælpe funktioner
impl<'a, CTX> Context<'a, CTX> {
    pub fn new(data: &'a mut CTX) -> Self {
        Context {
            ctx: data,
            sent: false,
        }
    }

    pub fn serialize(&self, message: impl Serialize) -> String {
        serde_json::to_string(&message).expect("failed to serialize")
    }

    pub fn sent(&self) -> bool {
        self.sent
    }
}

// Selve implementationen af Context for socket servere
impl<'a> MessageHandler for Context<'a, WebsocketContext<MyWebSocket>> {
    fn send(&mut self, message: impl Serialize) {
        let serialized = self.serialize(message);
        self.sent = true;
        self.ctx.text(serialized);
    }
}

// TODO! implementation af en http server context så vi også nemt kan svare den vej
