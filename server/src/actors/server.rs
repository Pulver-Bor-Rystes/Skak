use crate::{
    actors::engine::Engine,
    actors::game::{self, Game, TimeFormat},
    actors::session::{self, Session},
    std_format_msgs::OutgoingWsMsg,
};
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use std::{collections::HashMap, time::Duration};

use super::engine;

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
    engines: HashMap<String, Addr<Engine>>,
    games: HashMap<usize, GameData>,

    rng: ThreadRng,
}

impl Server {
    pub fn new() -> Self {
        // initialize engines!
        // let juules = Engine::new("engine/ChessPlusPlus").start();
        // let stockfish = Engine::new("stockfish/stockfish").start();

        let mut engines = HashMap::new();
        // engines.insert(
        //     "juules".to_string(),
        //     Engine::new("engine/ChessPlusPlus").start(),
        // );

        engines.insert(
            "stockfish".to_string(),
            Engine::new("stockfish/stockfish").start(),
        );

        Self {
            clients: HashMap::new(),
            engines,
            games: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn deploy_msg<M>(&mut self, ids: Vec<usize>, msg: M)
    where
        M: Serialize + std::clone::Clone + std::marker::Send + 'static,
    {
        for id in &ids {
            let client = self.clients.get_mut(id);

            match client {
                Some(client) => client
                    .addr
                    .do_send(session::API::DeployMessage(msg.clone())),
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
        let game = self
            .games
            .iter()
            .find(|(_, game)| game.has_player(username));

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
pub enum SendMessage<M: Serialize> {
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
        use SendMessage as Sm;
        match msg {
            Sm::Broadcast(msg) => {
                // find alle id'er
                let mut list_of_ids = vec![];
                for id in self.clients.keys() {
                    list_of_ids.push(id.clone());
                }

                // send til alle id'er / klienter
                self.deploy_msg(list_of_ids, msg.clone());
            }
            Sm::To(targets, msg) => {
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
        use UpdateSessionData as Usd;
        match msg {
            Usd::Connect(sess_addr) => {
                // Gemmer klienten, så vi altid kan kommunikere til den
                let id = self.rng.gen::<usize>();
                let client = SessionData::new(sess_addr);
                self.clients.insert(id, client);

                return Some(id);
            }
            Usd::Disconnect(id) => {
                if let Some(client) = self.clients.remove(&id) {
                    if client.is_logged_in() {
                        self.broadcast_active_players();
                    }
                }
            }
            Usd::LoggedIn(id, username) => {
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
pub enum UserAPI {
    /// String = Username
    RequestGameState(String),
    /// Parameter oversigt:
    /// 1. Spillerens brugernavn
    /// 2. Ønsket modstander som brugernavn
    /// 3. Tidsformat
    NewGame(String, String, TimeFormat),
}

impl Handler<UserAPI> for Server {
    type Result = bool;

    fn handle(&mut self, msg: UserAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserAPI::RequestGameState(username) => {
                let game = self.find_game(&username);

                if game.is_some() {
                    game.unwrap().addr.do_send(game::API::GetState(username));
                }
            }
            UserAPI::NewGame(p1, opponent, time_format) => {
                let id = self.rng.gen::<usize>();

                // er en af spillerene i gang med et spil?
                if self.find_game(&opponent).is_some() || self.find_game(&p1).is_some() {
                    return false;
                }

                let mut player2: Option<String> = None;

                // tjekker først om det er en engine vi vil spille imod!
                match self.engines.iter().find(|(name, _addr)| &&opponent == name) {
                    Some(_) => player2 = Some(opponent.clone()),
                    None => {}
                }

                // finder den første spiller der har det brugernavn
                match self
                    .clients
                    .iter()
                    .filter(|client| client.1.is_logged_in())
                    .find(|client| client.1.username.as_ref().unwrap() == &opponent)
                {
                    Some(_) => player2 = Some(opponent),
                    None => {}
                }

                if player2.is_none() {
                    return false;
                }

                let p2 = player2.unwrap();

                // start spillet
                let game = Game::new(&id, &ctx.address(), &p1, &p2, &time_format);

                // starter en actor og gemmer den :)
                let addr = game.start();
                self.games.insert(id, GameData { addr, p1, p2 });
            }
        }

        true
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub enum GameAPI {
    /// Fortæller brugeren eller engine at det er deres tur¨
    /// 0. game id
    /// 1. username
    /// 2. fen string
    /// 3. hvor meget tid spilleren har tilbage
    YourTurn(usize, String, String, Duration),
}

impl Handler<GameAPI> for Server {
    type Result = bool;

    fn handle(&mut self, msg: GameAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            GameAPI::YourTurn(game_id, username, fen, time_left) => {
                let user = self
                    .clients
                    .iter()
                    .find(|(_id, sesh_data)| sesh_data.username == Some(username.clone()));

                if user.is_some() {
                    let id = user.unwrap().0;
                    let last_move: Option<String> = None;

                    self.deploy_msg(
                        vec![id.to_owned()],
                        OutgoingWsMsg::content("your turn", last_move),
                    );

                    return true;
                }

                let engine = self
                    .engines
                    .iter()
                    .find(|(engine_name, _addr)| engine_name == &&username);

                if engine.is_some() {
                    let (_engine_name, addr) = engine.unwrap();
                    addr.send(engine::API::Search(fen, time_left))
                        .into_actor(self)
                        .then(move |res, act, ctx| {
                            match res {
                                Ok(chess_move) => {
                                    let game = act.games.get(&game_id);
                                    match game {
                                        Some(game) => {
                                            game.addr.do_send(game::API::Move(chess_move))
                                        }
                                        None => {}
                                    }
                                }
                                _ => ctx.stop(),
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                }
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
