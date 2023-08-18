use std::collections::HashMap;

use actix::prelude::*;
use serde::Serialize;

use super::session::{DeployMessage, Session};
use super::std_format_msgs::WrappedResult;
use rand::{self, rngs::ThreadRng, Rng};

struct Client {
    addr: Addr<Session>,
    username: Option<String>,
}

impl Client {
    fn new(addr: Addr<Session>) -> Self {
        Client {
            addr,
            username: None,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.username.is_some()
    }
}

pub struct Server {
    /// En liste over alle forbindelser uanset om de logget ind eller ej!
    clients: HashMap<usize, Client>,

    rng: ThreadRng,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn deploy_msg<M>(&mut self, ids: Vec<usize>, msg: M)
    where
        M: Serialize + std::clone::Clone + std::marker::Send + 'static + std::fmt::Debug,
    {
        for id in &ids {
            let client = self.clients.get_mut(id);

            match client {
                Some(client) => client.addr.do_send(DeployMessage::IntoJson(msg.clone())),
                None => {}
            }
        }
    }

    fn broadcast_active_players(&mut self) {
        let mut players = vec![];
        let mut ids = vec![];
        for (id, player) in self.clients.iter() {
            match &player.username {
                Some(username) => {
                    players.push(username.clone());
                    ids.push(id.clone());
                }
                None => {}
            }
        }

        self.deploy_msg(ids, WrappedResult::content("active_players", players));
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Ready to handle ws sessions!");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Server stopped");
    }
}

/// Et event andre `Actor`'s kan bruge til at sende en besked til en eller flere klienter
#[allow(dead_code)]
#[derive(Message, Debug)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub enum SendMessage<M: Serialize + std::fmt::Debug> {
    Broadcast(M),
    To(String, M),
}

impl<M> Handler<SendMessage<M>> for Server
where
    M: Serialize + std::clone::Clone + std::marker::Send + 'static + std::fmt::Debug,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: SendMessage<M>, _ctx: &mut Context<Self>) -> Self::Result {
        // println!("Recieved msg: {:?}", msg);

        // få fat i klienternes addr og bed dem sende en besked!
        match msg {
            SendMessage::Broadcast(msg) => {
                // find alle id'er
                let mut list_of_ids = vec![];
                for id in self.clients.keys() {
                    list_of_ids.push(id.clone());
                }

                // send til alle id'er / klienter
                self.deploy_msg(list_of_ids, msg.clone());
            }
            SendMessage::To(target, msg) => {
                let target = Some(target);
                let mut target_id = None;
                for (id, client) in self.clients.iter() {
                    if client.username == target {
                        target_id = Some(id.clone());
                    }
                }

                match target_id {
                    Some(id) => self.deploy_msg(vec![id], msg),
                    None => panic!("Could not find a valid id!"), // Hvis det her bliver et problem, kan det være at vi skal tillade at den bare ignorerer det
                }
            }
        }

        Ok(true)
    }
}

/// Et event så en `session` kan opdatere sig selv.
///
/// `Connect`, `Disconnect` eller `LoggedIn(username: String)`
/// Når en `session` først oprettets skal den blot registreres.
/// Når en session bliver valideret ved at en bruger logger ind,
/// sender sessionen en opdatering med det givne brugernavn som
/// unikt kan identificere en session.
///
/// Det tillader på sigt at andre sessions kan sende beskeder til
/// en bruger uden at kende brugerens session id!
///
/// Returnerer `id: usize`, som skal bruges når sessionen engang slutter.
#[derive(Message, Debug)]
#[rtype(result = "Option<usize>")]
pub enum UpdateSessionData {
    Connect(Addr<Session>),
    Disconnect(usize),
    LoggedIn(usize, String),
}

impl Handler<UpdateSessionData> for Server {
    type Result = Option<usize>;

    fn handle(&mut self, msg: UpdateSessionData, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            UpdateSessionData::Connect(sess_addr) => {
                // Gemmer klienten, så vi altid kan kommunikere til den
                let id = self.rng.gen::<usize>();
                let client = Client::new(sess_addr);
                self.clients.insert(id, client);

                return Some(id);
            }
            UpdateSessionData::Disconnect(id) => {
                if let Some(client) = self.clients.remove(&id) {
                    if client.is_logged_in() {
                        self.broadcast_active_players();
                    }
                }
            }
            UpdateSessionData::LoggedIn(id, username) => {
                if let Some(mut client) = self.clients.get_mut(&id) {
                    client.username = Some(username);

                    self.broadcast_active_players();
                }
            }
        }

        None
    }
}

// struct IncomingMessage
