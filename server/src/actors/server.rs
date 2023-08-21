use super::session::{DeployMessage, Session};
use crate::{
    actors::game::{self, Game},
    std_format_msgs::OutgoingWsMsg,
};
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
struct SessionData {
    addr: Addr<Session>,
    username: Option<String>,
}

impl SessionData {
    fn new(addr: Addr<Session>) -> Self {
        SessionData {
            addr,
            username: None,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.username.is_some()
    }
}

struct GameData {
    addr: Addr<Game>,
    p1: String,
    p2: String,
}

impl GameData {
    fn has_player(&self, username: &str) -> bool {
        &self.p1 == username || &self.p2 == username
    }
}

pub struct Server {
    /// En liste over alle forbindelser uanset om de logget ind eller ej!
    clients: HashMap<usize, SessionData>,
    games: HashMap<usize, GameData>,

    rng: ThreadRng,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            games: HashMap::new(),
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

        self.deploy_msg(ids, OutgoingWsMsg::content("active_players", players));
    }

    fn find_game(&self, username: &str) -> Option<&GameData> {
        let game = self.games.iter().find(|game| game.1.has_player(username));

        if game.is_some() {
            Some(game.unwrap().1)
        } else {
            None
        }
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
    To(Vec<String>, M),
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
            SendMessage::To(targets, msg) => {
                let mut id_list = vec![];

                self.clients.iter().for_each(|(id, sesh_data)| {
                    if sesh_data.username.is_none() {
                        return;
                    }

                    if !targets.contains(&sesh_data.username.as_ref().unwrap()) {
                        return;
                    }

                    id_list.push(id.clone());
                });

                self.deploy_msg(id_list, msg);
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
                let client = SessionData::new(sess_addr);
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

#[derive(Message)]
#[rtype(result = "bool")]
pub enum API {
    RequestGameState(String), // username
    NewGame(String, String),  // player1 and player2 username
}

impl Handler<API> for Server {
    type Result = bool;

    fn handle(&mut self, msg: API, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            API::RequestGameState(username) => {
                let game = self.find_game(&username);

                if game.is_some() {
                    game.unwrap().addr.do_send(game::API::GetState(username));
                }
            }
            API::NewGame(player1, player2) => {
                let id = self.rng.gen::<usize>();

                if self.find_game(&player1).is_some() || self.find_game(&player2).is_some() {
                    return false;
                }

                let p1 = self
                    .clients
                    .iter()
                    .filter(|client| client.1.is_logged_in())
                    .find(|client| client.1.username.as_ref().unwrap() == &player1);

                let p2 = self
                    .clients
                    .iter()
                    .filter(|client| client.1.is_logged_in())
                    .find(|client| client.1.username.as_ref().unwrap() == &player2);

                if p1.is_none() || p2.is_none() {
                    return false;
                }

                let p1 = p1.unwrap().1.username.as_ref().unwrap().clone();
                let p2 = p2.unwrap().1.username.as_ref().unwrap().clone();

                let game = Game::new(id.clone(), ctx.address().clone(), p1.clone(), p2.clone());

                let addr = game.start();
                self.games.insert(id, GameData { addr, p1, p2 });
            }
        }

        true
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub enum Cleanup {
    Game(usize), // fjerner et spil via dets id!
}

impl Handler<Cleanup> for Server {
    type Result = bool;

    fn handle(&mut self, msg: Cleanup, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Cleanup::Game(id) => {
                let _ = self.games.remove(&id);
            }
        }

        true
    }
}
