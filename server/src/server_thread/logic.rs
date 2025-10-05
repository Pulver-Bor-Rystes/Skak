use rand::Rng;
use serde::Serialize;
use sqlx::{MySql, Pool};
use crate::{client_thread::api::BrowserAPI, error, std_format_msgs::OutgoingWsMsg};
use super::*;




impl ServerThread {
    pub fn new(sql_pool: Pool<MySql>) -> Self {
        Self {
            clients: HashMap::new(),
            engines: HashMap::new(),
            games: HashMap::new(),
            rng: rand::thread_rng(),
            ids: Vec::new(),
            sql_conn: sql_pool,
        }
    }

    pub fn start_bots(&mut self, ctx: &mut Context<ServerThread>) -> Result<(), std::io::Error> {
        let bots = std::fs::read_dir("./bots")?;

        for bot in bots {
            let path = bot?.path().into_os_string().into_string().unwrap();
            let name = path.split("/").last().unwrap_or("NAMING GONE WRONG"); // burde kun kunne gå galt, hvis programmet køres på en windows maskine

            match EngineThread::new(&path, ctx.address()) {
                Some(actor) => {
                    self.engines.insert(name.into(), actor.start());
                }
                None => error!("When trying to load {path} an error occured?"),
            }
        }

        return Ok(());
    }


    pub fn is_player_in_game(&self, username: &str) -> bool {
        for (_, (p1, p2, _)) in &self.games {
            if p1 == username || p2 == username {
                return true;
            }
        }

        false
    }

    /// Fortæller alle forbundne klienter, hvilke brugere der er forbundet
    pub fn broadcast_active_players(&self) {
        let mut names = Vec::new();

        for (_, (username, _)) in &self.clients {
            if names.contains(username) {
                continue;
            }
            names.push(username.clone());
        }

        for (_, (username, client_thread)) in &self.clients {
            let mut names_clone = names.clone();
            names_clone.retain(|v| v != username);

            let msg = OutgoingWsMsg::content("active_users", names_clone);

            client_thread.do_send(BrowserAPI::Message(msg));
        }
    }

    pub fn send_to_client_browser<M>(&self, id: usize, msg: M)
    where
        M: Serialize + std::marker::Send + std::fmt::Debug + 'static,
    {
        if let Some((_, client_thread)) = self.clients.get(&id) {
            client_thread.do_send(BrowserAPI::Message(msg));
        }
    }

    pub fn send_to_clients_browser<M>(&self, username: String, msg: M)
    where
        M: Serialize + std::marker::Send + std::fmt::Debug + Clone + 'static,
    {
        for (_, (client_username, client_thread)) in &self.clients {
            if client_username == &username {
                client_thread.do_send(BrowserAPI::Message(msg.clone()));
            }
        }
    }

    pub fn gen_unique_id(&mut self) -> usize {
        loop {
            let id = self.rng.gen::<usize>();

            if !self.ids.contains(&id) {
                self.ids.push(id);
                return id;
            }
        }
    }

    pub fn remove_unique_id(&mut self, id: &usize) {
        self.ids.retain(|v| v != id);
    }
}
