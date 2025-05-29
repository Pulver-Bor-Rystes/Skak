use actix::prelude::*;
use rand::Rng;
use serde::Serialize;
use crate::{client_thread::{api::client_thread_api as ClientThreadAPI, ClientThread}, game_thread::{api::game_thread_api as GameThreadAPI, types::TimeFormat, GameThread}, std_format_msgs::OutgoingWsMsg};

use super::{ServerThread, Username};
use server_thread_api::*;

type ClientIdCallback = usize;
type MeUsername = String;
type OpponentUsername = String;


pub mod server_thread_api {
    use super::*;

    impl Message for CommandsAPI { type Result = bool; }
    pub enum CommandsAPI {
        ClientLogin(Username, Addr<ClientThread>),
    }

    impl<M> Message for ToClientBrowserAPI<M> where M: Serialize + std::marker::Send + std::fmt::Debug { type Result = bool; }
    pub enum ToClientBrowserAPI<M> where M: Serialize + std::marker::Send + std::fmt::Debug {
        Message(usize, M),
    }

    impl Message for ClientCommandsAPI { type Result = bool; }
    pub enum ClientCommandsAPI {
        LeaveGame(String),
    }


    impl Message for PassthroughGameAPI { type Result = bool; }
    pub enum PassthroughGameAPI {
        NewGame(MeUsername, OpponentUsername, TimeFormat),
        /// ID viser hvilken klient serveren skal svare tilbage til
        GetBots(ClientIdCallback),
        RequestGameState(ClientIdCallback, MeUsername),
    }

}



impl Handler<CommandsAPI> for ServerThread {
    type Result = bool;

    fn handle(&mut self, msg: CommandsAPI, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            CommandsAPI::ClientLogin(username, client_thread) => {
                let id = self.rng.gen::<usize>();
                
                // Gemmer klienten og sender dens id til thread
                self.clients.insert(id, (username.clone(), client_thread.clone()));
                client_thread.do_send(ClientThreadAPI::IdentifierAPI::Set(id));

                // fortæller alle hvor mange spillere logget på!
                self.broadcast_active_players();

                // Tjek om den er med i et spil og fortæl klienten
                if self.is_player_in_game(&username) {
                    client_thread.do_send(ClientThreadAPI::GameAPI::SetInGame(true));
                }
                
                true
            },
        }
    }
}


impl<M> Handler<ToClientBrowserAPI<M>> for ServerThread where M: Serialize + std::marker::Send + std::fmt::Debug + 'static {
    type Result = bool;

    fn handle(&mut self, msg: ToClientBrowserAPI<M>, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            ToClientBrowserAPI::Message(client_id, msg) => self.send_to_client_browser(client_id, msg),
        };

        true
    }
}


impl Handler<ClientCommandsAPI> for ServerThread {
    type Result = bool;

    fn handle(&mut self, msg: ClientCommandsAPI, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            ClientCommandsAPI::LeaveGame(username) => {
                for (_, (client_username, client_thread)) in &self.clients {
                    if client_username != &username { continue }
                    client_thread.do_send(ClientThreadAPI::GameAPI::SetInGame(false));
                }
            }
        };

        true
    }
}


impl Handler<PassthroughGameAPI> for ServerThread {
    type Result = bool;

    fn handle(&mut self, msg: PassthroughGameAPI, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PassthroughGameAPI::NewGame(me_username, opponent_username, time_format) => {
                // ingen af spillerne må være i gang med et spil!
                if self.is_player_in_game(&me_username) { return false }
                if self.is_player_in_game(&opponent_username) { return false }

                let id = self.gen_unique_id();
                let bool = self.rng.gen_bool(0.5);

                let (white, black) = match bool {
                    true => (me_username, opponent_username),
                    false => (opponent_username, me_username),
                };

                // Starter Thread
                let game = GameThread::new(id, ctx.address(), &white, &black, time_format).start();
                self.games.insert(id, (white.clone(), black.clone(), game));

                // Fortæller brugerne at de er i et spil
                for (_, (username, client_thread)) in &self.clients {
                    if username != &white && username != &black { continue }

                    client_thread.do_send(ClientThreadAPI::GameAPI::SetInGame(true));
                }

                true
            },
            PassthroughGameAPI::GetBots(id_to_answer) => {
                let mut engine_names = Vec::new();

                for (engine_name, _) in &self.engines {
                    engine_names.push(engine_name.clone());
                }

                self.send_to_client_browser(id_to_answer, OutgoingWsMsg::content("engines", engine_names));

                true
            },
            PassthroughGameAPI::RequestGameState(id_to_answer, username) => {
                let game = self
                    .games
                    .iter()
                    .find(|(_, (p1, p2, _))| {
                        *p1 == username || *p2 == username
                    });

                if let Some((_, (_, _, game_thread))) = game {
                    game_thread.do_send(GameThreadAPI::CommandsAPI::RequestGameState(id_to_answer));
                }

                true
            },
        }
    }
}


