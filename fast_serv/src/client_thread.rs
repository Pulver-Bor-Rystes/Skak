use crate::{
    errors::RequestError,
    parsing::{Json, Message, RequestResp},
    server_thread::ServerApi,
    types::Username,
};
use jsonic::json_item::JsonItem;
use log::{debug, error, info};
use mysql::{Pool, params, prelude::Queryable};
use rand::{distr::Alphanumeric, prelude::*};
use std::{
    error::Error,
    net::TcpStream,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::Duration,
};
use tungstenite::WebSocket;
use uuid::Uuid;

pub struct Client {
    uuid: Uuid,
    /// bruges af andre threads, til at kommunikerer med denne client
    client_api: Sender<ClientApi>,
    /// server_api bruges til at snakke med resten af serveren.
    server_api: Sender<ServerApi>,
    /// sql pool connection
    pool: Pool,
    state: ClientState,
}

enum ClientState {
    LoggedIn(UserData),
    NoState,
}

struct UserData {
    username: Username,
}

pub enum ClientApi {
    SendJSON(String),
}

impl Client {
    fn get_username(&self) -> Option<&Username> {
        match &self.state {
            ClientState::LoggedIn(user_data) => Some(&user_data.username),
            _ => None,
        }
    }

    fn register_login(&mut self, username: &str) {
        // gem brugernavnet
        self.state = ClientState::LoggedIn(UserData {
            username: username.into(),
        });

        // fortæl serveren at vi har en ny bruger!
        self.server_api
            .send(ServerApi::RegisterClientLogin(
                self.uuid,
                username.into(),
                self.client_api.clone(),
            ))
            .expect("undersøg hvorfor det her går galt");
    }

    fn login(&mut self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let Some(username) = json["content"]["username"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };
        let Some(password) = json["content"]["password"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };

        let result: Option<String> = conn.exec_first(
            "SELECT hash FROM Password WHERE owner = :owner",
            params! {
                "owner" => username,
            },
        )?;

        match result {
            Some(hash) => {
                if !pwhash::bcrypt::verify(password, &hash) {
                    return Err(RequestError::WrongPassword.into());
                }

                let cookie: String = rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(64)
                    .map(char::from)
                    .collect();

                let cookie_hash = pwhash::bcrypt::hash(&cookie)?;

                // gem i database
                conn.exec_drop(
                    "INSERT INTO Cookie (hash, owner) VALUES (:hash, :owner)",
                    params! {
                        "hash" => cookie_hash,
                        "owner" => username,
                    },
                )?;

                self.register_login(username);

                Ok(Box::new(RequestResp::Cookie(cookie)))
            }
            None => Err(RequestError::NoUserFound.into()),
        }
    }

    fn login_with_cookie(&mut self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let Some(username) = json["content"]["username"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };
        let Some(cookie) = json["content"]["cookie"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };

        let hashes: Vec<String> = conn
            .exec(
                "SELECT hash FROM Cookie WHERE owner = :owner",
                params! {
                    "owner" => username,
                },
            )
            .unwrap_or(Vec::new());

        let mut found_cookie = false;
        for hash in hashes {
            found_cookie = found_cookie || pwhash::bcrypt::verify(cookie, &hash);
        }

        if !found_cookie {
            return Err(Box::new(RequestError::WrongPassword));
        }

        // cookien virker
        // vi registrer det på serveren og brugeren skal ikke gøre mere
        self.register_login(username);

        Ok(Box::new(RequestResp::AllGood))
    }

    // fn signup() -> Result<(), Box<dyn Error>> { Ok(()) }
    fn whats_my_rating(&mut self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        match &self.state {
            ClientState::NoState => Err(Box::new(RequestError::NotLoggedIn)),
            ClientState::LoggedIn(data) => {
                let mut conn = self.pool.get_conn()?;

                let rating: Option<i32> = conn.exec_first(
                    "select rating from User where username = :user",
                    params! {
                        "user" => data.username.as_str(),
                    },
                )?;

                match rating {
                    Some(val) => Ok(Box::new(RequestResp::Rating(val))),
                    None => Err(Box::new(RequestError::RatingNotFound)),
                }
            }
        }
    }

    fn active_users(&mut self, _json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        self.server_api
            .send(ServerApi::QueryActiveUsers(self.uuid))?;
        Ok(Box::new(RequestResp::IsBeingProcessed))
    }

    fn query_engines(&mut self, _: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        self.server_api.send(ServerApi::QueryEngines(self.uuid))?;

        Ok(Box::new(RequestResp::IsBeingProcessed))
    }

    fn query_friends(&mut self, _: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let username = match &self.state {
            ClientState::LoggedIn(uname) => uname.username.to_string(),
            ClientState::NoState => "".to_string(),
        };

        let result: Vec<(String, String)> = conn
            .exec(
                "SELECT user1, user2 FROM Friendship WHERE user1 = :username or user2 = :username",
                params! {
                    "username" => &username.to_string(),
                },
            )
            .unwrap_or(Vec::new());

        // collect the friend from the list with the type Username
        let friends: Vec<Username> = result
            .iter()
            .map(|(u1, u2)| {
                if u1 == &username.to_string() {
                    Username::new(u2.clone())
                } else {
                    Username::new(u1.clone())
                }
            })
            .collect();

        Ok(Box::new(RequestResp::List(friends)))
    }

    fn handle(&mut self, json: JsonItem) -> String {
        // Get topic
        let Some(topic) = json["topic"].as_str() else {
            return Message {
                topic: "no topic".into(),
                result: false,
                content: Box::new(format!("{}: {:?}", RequestError::TopicParsingFailed, json)),
            }
            .to_json();
        };

        // rules
        // Bliver smidt hen til specifik funktion

        let res: Result<Box<dyn Json>, Box<dyn std::error::Error>> = match self.state {
            ClientState::LoggedIn(_) => {
                match topic {
                    // queries
                    "query:engines" => self.query_engines(&json),
                    "query:my_rating" => self.whats_my_rating(&json),
                    "query:friends" => self.query_friends(&json),

                    // friendship stuff
                    "friend:invite" => todo!(),
                    "friend:cancel_invite" => todo!(),
                    "friend:accept" => todo!(),
                    "friend:reject" => todo!(),
                    "friend:remove" => todo!(),

                    // game stuff
                    "game:invite" => self.game_invite(&json),
                    "game:cancel_invite" => self.game_cancel_invite(&json),
                    "game:accept" => self.accept_game_invite(&json),
                    "game:move" => self.game_play_move(&json),

                    _ => Err(RequestError::RequestHadNoHandler.into()),
                }
            }
            ClientState::NoState => {
                match topic {
                    // login!
                    "login:psw" => self.login(&json),
                    "login:cookie" => self.login_with_cookie(&json),

                    _ => Err(RequestError::RequestHadNoHandler.into()),
                }
            }
        };

        // Konstruerer beskeden som skal tilbage til Browser

        match res {
            Ok(msg) => Message {
                topic: topic.into(),
                result: true,
                content: msg,
            }
            .to_json(),
            Err(err) => Message {
                topic: topic.into(),
                result: false,
                content: Box::new(format!("{err}")),
            }
            .to_json(),
        }
    }

    pub fn new(websocket: WebSocket<TcpStream>, server_api: Sender<ServerApi>, pool: Pool) {
        let uuid = Uuid::new_v4();
        let ws = Arc::new(Mutex::new(websocket));

        // En API til at sende JSON beskeder til browseren
        // JSON input skal allerede være formatteret!
        let (tx, rx) = mpsc::channel::<ClientApi>();
        let ws_stopped = Arc::new(AtomicBool::new(false));

        let mut client = Client {
            uuid,
            client_api: tx.clone(),
            server_api,
            pool,
            state: ClientState::NoState,
        };

        // send thread
        let ws1 = ws.clone();
        let should_close = ws_stopped.clone();
        thread::spawn(move || {
            loop {
                match rx.try_recv() {
                    Err(_rx_err) => {}
                    Ok(ClientApi::SendJSON(msg)) => {
                        // Locker websocket, så vi kan sende
                        if let Ok(mut ws) = ws1.lock() {
                            // sender og tjekker for fejl!
                            if let Err(ws_err) = ws.send(msg.into()) {
                                error!(target: "C Send", "{ws_err}");
                            }

                            drop(ws);
                        } else {
                            // burde ikke ske!
                            error!(target: "C Send", "Failed to obtain WS lock :/");
                        }
                    }
                }

                // lukker thread, hvis den har fået besked på det!
                if should_close.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        // receive thread
        thread::spawn(move || {
            loop {
                if let Ok(mut ws) = ws.lock() {
                    match ws.read() {
                        Ok(msg) => {
                            // introducér api
                            match jsonic::parse(&msg.to_string()) {
                                Ok(json) => {
                                    let msg = client.handle(json);
                                    debug!("sending msg: {msg}");

                                    match tx.send(ClientApi::SendJSON(msg)) {
                                        Ok(_) => {}
                                        Err(err) => error!("{err}"),
                                    }
                                }
                                Err(_error) => {} // ignorer alt der fejler!
                            }
                        }
                        Err(err) => match err {
                            tungstenite::Error::ConnectionClosed => break,
                            tungstenite::Error::AlreadyClosed => break,
                            tungstenite::Error::AttackAttempt => break,
                            _ => {}
                        },
                    }
                    // drop(ws);
                }

                // det er åbenbart ret vigtigt at vente nogle få ms. Ellers går thread amok med cpu'en hahaha
                thread::sleep(Duration::from_millis(10));
            }

            // tell server api to drop
            ws_stopped.store(true, Ordering::Relaxed);

            match client.state {
                ClientState::LoggedIn(user) => {
                    let _ = client
                        .server_api
                        .send(ServerApi::ClientDestroy(user.username, uuid));
                }
                _ => {}
            }
        });
    }

    fn game_invite(&self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let Some(target_user) = json["content"]["user"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };

        match &self.state {
            ClientState::LoggedIn(user) => {
                self.server_api.send(ServerApi::GameInvite(
                    user.username.clone().into(),
                    target_user.into(),
                ))?;
                Ok(Box::new(RequestResp::IsBeingProcessed))
            }
            ClientState::NoState => Err("Not logged in".into()),
        }
    }

    fn accept_game_invite(&self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let Some(target_hash) = json["content"]["hash"].as_str() else {
            return Err(Box::new(RequestError::JsonParsingError));
        };
        let Ok(hash) = target_hash.parse::<u64>() else {
            return Err(Box::new(RequestError::JsonParsingError));
        };

        let api_call = ServerApi::GameAcceptInvite(self.get_username().unwrap().to_owned(), hash);
        self.server_api.send(api_call)?;
        Ok(Box::new(RequestResp::IsBeingProcessed))
    }

    fn game_play_move(&self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let Some(target_hash) = json["content"]["hash"].as_str() else {
            return Err(Box::new(RequestError::JsonParsingError));
        };
        let Ok(hash) = target_hash.parse::<u64>() else {
            return Err(Box::new(RequestError::JsonParsingError));
        };
        let Some(move_name) = json["content"]["move_name"].as_str() else {
            return Err(Box::new(RequestError::JsonParsingError));
        };

        let api_call = ServerApi::GameMove(self.uuid, hash, move_name.into());
        self.server_api.send(api_call)?;
        Ok(Box::new(RequestResp::IsBeingProcessed))
    }

    fn game_cancel_invite(&self, json: &JsonItem) -> Result<Box<dyn Json>, Box<dyn Error>> {
        let Some(target_hash) = json["content"]["hash"].as_str() else {
            return Err(RequestError::JsonParsingError.into());
        };

        // hash to u64
        if let Ok(hash) = target_hash.parse::<u64>() {
            match &self.state {
                ClientState::LoggedIn(user) => {
                    self.server_api
                        .send(ServerApi::GameCancelInvite(hash, user.username.clone()))?;
                    Ok(Box::new(RequestResp::IsBeingProcessed))
                }
                ClientState::NoState => Err("Not logged in".into()),
            }
        } else {
            return Err(RequestError::JsonParsingError.into());
        }
    }
}
