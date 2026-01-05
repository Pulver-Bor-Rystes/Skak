use crate::types::Username;
use std::fmt::Debug;

pub trait Json {
    fn to_json(&self) -> String;
    fn prepare_json(&self) -> String {
        let json_txt = self.to_json();

        if json_txt.starts_with('"') && json_txt.ends_with('"') {
            json_txt[1..json_txt.len() - 1].to_string()
        } else {
            json_txt
        }
    }
}

/// Message er en struct til at lave beskeder, som på et tidspunkt skal sendes til klienten
pub struct Message {
    pub topic: String,
    pub result: bool,
    pub content: Box<dyn Json>,
}

impl Json for Message {
    fn to_json(&self) -> String {
        format!(
            "{{\"topic\": {:#?}, \"result\": {}, \"content\": {:#?}}}",
            self.topic,
            self.result,
            self.content.to_json(),
        )
    }
}

impl Json for bool {
    fn to_json(&self) -> String {
        format!("{self}")
    }
}

impl Json for i32 {
    fn to_json(&self) -> String {
        format!("{self}")
    }
}

impl Json for u32 {
    fn to_json(&self) -> String {
        format!("{self}")
    }
}

impl Json for u64 {
    fn to_json(&self) -> String {
        format!("\"{self}\"")
    }
}

impl Json for [Username; 2] {
    fn to_json(&self) -> String {
        format!("[{}, {}]", self[0].to_json(), self[1].to_json())
    }
}

impl Json for usize {
    fn to_json(&self) -> String {
        format!("{self}")
    }
}

impl<T: Json> Json for Vec<T> {
    fn to_json(&self) -> String {
        let mut s = String::new();

        s += "[";
        for i in 0..self.len() {
            let v = self[i].to_json();
            s += &v;
            if i < self.len() - 1 {
                s += ", ";
            }
        }
        s += "]";
        s
    }
}

#[derive(Debug)]
pub enum RequestResp {
    /// Bliver brugt, når en request bliver smidt over til
    /// serveren, som senere kan give besked til client
    /// at den skal sende noget til browseren
    /// kort sagt: request modtaget og bliver fortsat behandlet
    Pending,
    InviteSent,
    Cookie(String),
    AllGood,
    /// Fortæller klienten at requesten er under behandling
    IsBeingProcessed,

    /// bliver brugt til lidt af hvert
    /// primært når vi henter aktive brugere og bots
    List(Vec<Username>),
    // InviteList(Vec<(u64, Username, Username)>),
    SignupSuccesful,
    Rating(i32),
    Value(String, String),

    // Game stuff
    /// Brug den til at fortælle en klient at de har fået en invitation til et spil
    GameInviteReceived(Username),
    GameInviteSent(Username),
    /// Samme idé, men til accept
    GameInviteAccepted(Username),
    /// Samme shit, men til afvisning
    GameInviteDeclined(Username),
}

impl From<RequestResp> for String {
    fn from(value: RequestResp) -> Self {
        value.to_json()
    }
}

impl Json for RequestResp {
    fn to_json(&self) -> String {
        match self {
            Self::Cookie(str) => format!("{{\"cookie\": {}}}", str.to_json()), // "cookie": "abcde"
            Self::Rating(num) => format!("{{\"rating\": {}}}", num.to_json()),
            Self::List(vec) => format!("{{\"list\": {}}}", vec.to_json()),
            Self::Value(field, str) => format!("{{\"{}\": {}}}", field, str),
            Self::GameInviteAccepted(username) => {
                format!("{{\"invite_accepted\": {}}}", username.to_json())
            }
            Self::GameInviteDeclined(username) => {
                format!("{{\"invite_declined\": {}}}", username.to_json())
            }
            Self::GameInviteSent(username) => {
                format!("{{\"invite_sent\": {}}}", username.to_json())
            }
            Self::GameInviteReceived(username) => {
                format!("{{\"invite_received\": {}}}", username.to_json())
            }
            e => format!("\"{e:?}\""),
        }
    }
}

impl Json for Vec<(u64, Username, Username)> {
    fn to_json(&self) -> String {
        let mut str = String::new();

        str += "[";

        let mut index = 0;
        for (hash, from, to) in self {
            if index > 0 {
                str.push_str(", ");
            }

            str.push_str(&format!(
                "{{\"hash\": {}, \"from\": {}, \"to\": {}}}",
                hash.to_string().to_json(),
                from.to_json(),
                to.to_json()
            ));

            index += 1;
        }

        str += "]";

        str
    }
}

impl Json for String {
    fn to_json(&self) -> String {
        format!("\"{self}\"")
    }
}
