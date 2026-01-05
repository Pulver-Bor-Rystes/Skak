use std::{collections::HashMap, fmt::Display, hash::{DefaultHasher, Hash, Hasher}, sync::mpsc::{self, Receiver, Sender}, time::Duration};
use serde::Serialize;
use crate::{error, info, threads::client::{ClientApi, Output}, to_json, TopicContent};



mod send_state;




pub struct Username(String);

impl Username {
    pub fn from(value: impl ToString) -> Self {
        Username (value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}





struct EngineApi;



pub enum ServerApi {
    // ClientNew(mpsc::Sender<ClientApi>), // bruges ikke alligevel
    ClientLogin(usize, Username, Sender<ClientApi>),
    ClientDestroy(Username, usize),
    /// From, To
    Invite(String, String),
    QueryBots(Username),
    QueryActiveUsers(Username),

    /// 1. Hvordan og hvilken slags data jeg vil modtage
    /// 2. Hvilken slags data jeg gerne vil ha'
    FetchData(Sender<Answer>, FetchApi),
}


pub enum FetchApi {
    BotNames,
    ActiveUsers,
}

#[derive(Debug)]
pub enum Answer {
    // BotNames(Vec<String>),
    // Users(Vec<String>),

    List(Vec<String>),
}


#[derive(Debug)]
struct Client {
    api: Sender<ClientApi>,
    id: usize,
}

pub struct Server {
    rx: Receiver<ServerApi>,
    tx: Sender<ServerApi>,
    /// usize: client id
    /// Sender<ClientApi> giver vel lidt sig selv
    clients: HashMap<String, Vec<Client>>,
    engines: HashMap<String, Sender<EngineApi>>,
    /// FromTo_Hash, (From, To)
    invites: HashMap<u64, (String, String)>,
    latest_id: usize,
}


#[derive(Serialize, Debug)]
struct Bot {
    name: String,
    rating: i32,
}


impl Server {
    pub fn new() -> Sender<ServerApi> {
        let (tx, rx) = mpsc::channel::<ServerApi>();
        
        let mut server = Server {
            rx,
            tx: tx.clone(),
            clients: HashMap::new(),
            engines: HashMap::new(),
            invites: HashMap::new(),
            latest_id: 0,
        };

        std::thread::spawn(move || server.server_thread());

        tx
    }


    fn send_to_user<T: Serialize>(&self, username: &str, msg: T) {
        if let Some(clients) = self.clients.get(username) {
            for c in clients {
                match serde_json::to_string(&msg) {
                    Ok(json) => c.api.send(ClientApi::SendJSON(json)).expect("kunne ikke sende json til client"),
                    Err(err) => error!("Noget gik galt: {err}"),
                }
            }
        }
        else {
            error!("Brugeren findes ikke {username}");
        }
    }


    fn update_whos_online(&self) {
        let mut list = Vec::new();

        for k in self.clients.keys() {
            list.push(k.clone());
        }

        for username in &list {
            let mut list = list.clone();
            list.retain(|u| u != username);

            let msg = TopicContent {
                topic: "query_active_users".into(),
                result: true,
                content: Output::List(list),
            };
            self.send_to_user(username, msg);
        }
    }

    
    pub fn server_thread(&mut self) {
        info!("hello from server thread");

        loop {
            let msg = self.rx.try_recv();
            if let Ok(msg) = msg {
                match msg {
                    ServerApi::ClientLogin(id, username, api) => {
                        // hvis brugeren ikke eksistere, så gemmer vi en ny
                        if let Some(client) = self.clients.get_mut(username.value()) {
                            client.push(Client {
                                api,
                                id,
                            });
                        }
                        else {
                            self.clients.insert(username.0.clone(), vec![Client {
                                api,
                                id,
                            }]);
                        }

                        self.update_whos_online();
                    }
                    ServerApi::Invite(from, to) => {
                        let mut hasher = DefaultHasher::new();

                        // laver et hash, som bruges til at identificere invitationen
                        let combined = format!("{from}, {to}");
                        combined.hash(&mut hasher);
                        let c_hash = hasher.finish();

                        // hvis invi allerede eksisterer, stop op og fortæl browser
                        let exists_msg = TopicContent { topic: "invite".into(), result: false, content: to.clone() };
                        if self.invites.contains_key(&c_hash) {
                            self.send_to_user(&from, exists_msg);
                            continue;
                        }

                        // gemmeer invitationen
                        self.invites.insert(c_hash, (from.clone(), to.clone()));

                        // fortæller modtageren at de har fået en invitation.
                        // let msg = TopicContent {
                        //     topic: "invite_receive".into(),
                        //     result: true,
                        //     content: from,
                        // };
                        
                        // self.send_to_user(to, msg);

                        let users = vec![Username::from(from), Username::from(to)];
                        for u in users {
                            self.invite_overview(u);
                        }
                    }
                    ServerApi::ClientDestroy(username, id) => {
                        let mut remove_client_all_together = false;
                        if let Some(clients) = self.clients.get_mut(username.value()) {
                            clients.retain(|c| c.id != id);

                            remove_client_all_together = clients.is_empty();
                        }


                        if remove_client_all_together {
                            self.clients.remove(username.value());
                            self.invites.retain(|_hash, (from, _to)| from != username.value());
                        }


                        
                        self.update_whos_online();
                    },
                    ServerApi::QueryBots(username) => {
                        // find bots
                        let bots: Vec<Bot> = vec![Bot { name: "stockfish".into(), rating: 3000 }, Bot { name: "sisyphus".into(), rating: 1800 }];
                        let msg = TopicContent {
                            topic: "getbots".into(),
                            result: true,
                            content: bots,
                        };

                        self.send_to_user(username.value(), msg);
                    },
                    ServerApi::QueryActiveUsers(username) => {
                        let clients = self.clients.keys().filter(|c| *c != username.value());
                        let mut list: Vec<String> = Vec::new();
                        for c in clients { list.push(c.to_owned()); }
                       
                        let msg = TopicContent {
                            topic: "query_active_users".into(),
                            result: true,
                            content: list,
                        };

                        self.send_to_user(username.value(), msg);
                    },
                    ServerApi::FetchData(tx, api) => {
                        let r = match api {
                            FetchApi::BotNames => {
                                let mut list = Vec::new();
                                self.engines.keys().for_each(|e| list.push(e.clone()));

                                tx.send(Answer::List(list))
                            },
                            FetchApi::ActiveUsers => {
                                let mut list = Vec::new();
                                self.clients.keys().for_each(|c| list.push(c.clone()));

                                tx.send(Answer::List(list))
                            },
                        };

                        match r {
                            Err(e) => panic!("{e}"),
                            _ => {},
                        }
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    }
}




fn server_send_invi_status(server: &Server, target: String) {
    
}
