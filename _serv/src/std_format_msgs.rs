use serde::{Deserialize, Serialize};


/// Skal bruges til at sende beskeder tilbage til klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsOut<M = serde_json::Value> {
    pub topic: String,
    pub payload: Payload<M>,
}

/// Skal bruges til at forstå beskeder fra klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsIn<M = serde_json::Value> {
    pub topic: String,
    pub content: M,
}

/// Skal bruges til at forstå beskeder fra klienten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTopic {
    pub topic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload<M = serde_json::Value> {
    pub result: bool,
    pub content: M,
}



impl<M> WsOut<M> where M: Serialize + std::marker::Send + std::fmt::Debug {
    /// Laver en ny [WsOut] med givent content, hvor result er true
    pub fn with_content(topic: impl ToString, content: M) -> WsOut<M> {
        WsOut {
            topic: topic.to_string(),
            payload: Payload {
                result: true,
                content,
            },
        }
    }

    /// Laver en ny [WsOut] med givent content, hvor result er false
    pub fn with_error(topic: impl ToString, content: M) -> WsOut<M> {
        WsOut {
            topic: topic.to_string(),
            payload: Payload {
                result: false,
                content,
            },
        }
    }

    /// Laver [WsOut] om til JSON
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize")
    }
}




// En masse templates når man skal parse en besked fra klienten
pub mod json_templates {
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

    #[derive(Deserialize, Debug, Clone)]
    pub struct PlayMove {
        pub chess_move: String,
    }

}
