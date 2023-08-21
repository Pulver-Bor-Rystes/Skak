use serde::{Deserialize, Serialize};

/// Skal bruges til at sende beskeder tilbage til klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrappedResult<M = String> {
    pub topic: String,
    pub payload: Payload<M>,
}

/// Skal bruges til at forstå beskeder fra klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrappedContent<M = String> {
    pub topic: String,
    pub content: M,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload<M = String> {
    pub result: bool,
    pub content: M,
}

impl<M> WrappedResult<M>
where
    M: Serialize + std::marker::Send + std::fmt::Debug,
{
    pub fn content(topic: impl ToString, content: M) -> WrappedResult<M> {
        WrappedResult {
            topic: topic.to_string(),
            payload: Payload {
                result: true,
                content,
            },
        }
    }

    pub fn error(topic: impl ToString, content: M) -> WrappedResult<M> {
        WrappedResult {
            topic: topic.to_string(),
            payload: Payload {
                result: false,
                content,
            },
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize")
    }
}

/// For deserialization, når vi kun vil kende topic!
/// Bruges kun én gang!
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TopicMsg {
    pub topic: String,
    pub content: serde_json::Value,
}

// En masse templates når man skal parse en besked fra klienten
pub mod content_templates {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Username {
        pub username: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Login {
        pub username: String,
        pub password: String,
    }
}
