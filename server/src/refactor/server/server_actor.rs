use std::ops::Add;

use sockets::socket_api;

use super::*;


pub struct Server {
    /// En liste over alle forbindelser uanset om de logget ind eller ej!
    pub clients: HashMap<usize, SessionData>,
    pub engines: HashMap<String, Addr<Engine>>,
    pub games: HashMap<usize, GameData>,

    pub rng: ThreadRng,
}


impl Server {
    pub fn new() -> Self {
        // initialize engines!
        // let juules = Engine::new("engine/ChessPlusPlus").start();
        // let stockfish = Engine::new("stockfish/stockfish").start();

        let mut engines = HashMap::new();
        engines.insert(
            "juules".to_string(),
            Engine::new("engine/ChessPlusPlus").start(),
        );

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

    pub fn deploy_msg<M>(&mut self, ids: Vec<usize>, msg: M)
    where
        M: Serialize + std::clone::Clone + std::marker::Send + 'static,
    {
        for id in &ids {
            let client = self.clients.get_mut(id);

            match client {
                Some(client) => client
                    .addr
                    .do_send(SocketAPI::SendDirectMessage(msg.clone())),
                None => {}
            }
        }
    }

    pub fn broadcast_active_players(&mut self) {
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

    pub fn find_game(&self, username: &str) -> Option<&GameData> {
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

    fn find_clients_addr(&self, username: &str) -> Vec<Addr<SocketSession>> {
        let mut clients = Vec::new();
        
        for client in &self.clients {
            if client.1.username == Some(username.to_string()) {
                clients.push(client.1.addr.clone())
            }
        }

        clients
    }

    fn find_clients_id(&self, username: &str) -> Vec<usize> {
        let mut clients = Vec::new();
        
        for client in &self.clients {
            if client.1.username == Some(username.to_string()) {
                clients.push(client.0.clone())
            }
        }

        clients
    }

    /// Fortæller klienten hvilket spil de er i gang med!
    pub fn set_game_addr(&self, username: &str, game: Addr<Game>) {
        let addrs = self.find_clients_addr(username);

        for client_addr in addrs {
            client_addr.do_send(SocketSessionAPI::UpdateGameAddr(game.clone()));
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