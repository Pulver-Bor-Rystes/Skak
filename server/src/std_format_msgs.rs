use serde::{Deserialize, Serialize};

/// Skal bruges til at sende beskeder tilbage til klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutgoingWsMsg<M = serde_json::Value> {
    pub topic: String,
    pub payload: ResultContent<M>,
}

/// Skal bruges til at forstå beskeder fra klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingWsMsg<M = serde_json::Value> {
    pub topic: String,
    pub content: M,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultContent<M = serde_json::Value> {
    pub result: bool,
    pub content: M,
}

impl<M> OutgoingWsMsg<M>
where
    M: Serialize + std::marker::Send + std::fmt::Debug,
{
    /// Laver en ny OutgoingWsMsg med givent content, hvor result er true
    pub fn content(topic: impl ToString, content: M) -> OutgoingWsMsg<M> {
        OutgoingWsMsg {
            topic: topic.to_string(),
            payload: ResultContent {
                result: true,
                content,
            },
        }
    }

    /// Laver en ny OutgoingWsMsg med givent content, hvor result er false
    pub fn error(topic: impl ToString, content: M) -> OutgoingWsMsg<M> {
        OutgoingWsMsg {
            topic: topic.to_string(),
            payload: ResultContent {
                result: false,
                content,
            },
        }
    }

    /// Laver OutgoingWsMsg om til JSON
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize")
    }
}

// En masse templates når man skal parse en besked fra klienten
pub mod content_templates {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Username {
        pub username: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct NewGame {
        pub username: String,
        pub timeformat: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Login {
        pub username: String,
        pub password: String,
    }

    impl Login {
        pub fn new(username: impl ToString, password: impl ToString) -> Self {
            Login {
                username: username.to_string(),
                password: password.to_string(),
            }
        }
    }
}
