use std::time::Duration;
use actix::prelude::*;
use rand::Rng;
use serde::Serialize;
use crate::{client_thread::{self, ClientThread}, engine_thread::api::EngineThreadAPI, game_thread::{self, types::TimeFormat, GameThread}, std_format_msgs::OutgoingWsMsg};
use super::{ServerThread, Username};


type ClientIdCallback = usize;
type MeUsername = String;
type OpponentUsername = String;

type FenString = String;
type GameID = usize;



#[derive(Message)]
#[rtype(result="bool")]
pub enum CommandsAPI {
    ClientLogin(Username, Addr<ClientThread>),
    RemoveClient(usize),
    RemoveEngine(String),
    RemoveGame(usize),
}

#[derive(Message)]
#[rtype(result="bool")]
pub enum ToClientBrowserAPI<M> where M: Serialize + std::marker::Send + std::fmt::Debug {
    MessageToClientID(usize, M),
    MessageToUsername(Username, M),
}

#[derive(Message)]
#[rtype(result="bool")]
pub enum ClientCommandsAPI {
    LeaveGame(Username),
    NotifyYourTurn(GameID, Username, FenString),
}

#[derive(Message)]
#[rtype(result="bool")]
pub enum GameCommandsAPI {
    NewGame(MeUsername, OpponentUsername, TimeFormat),
    /// ID viser hvilken klient serveren skal svare tilbage til
    GetBots(ClientIdCallback),
    RequestGameState(ClientIdCallback, Username),
    PlayMove(Username, String),
}



impl Handler<CommandsAPI> for ServerThread {
    type Result = bool;

    fn handle(&mut self, msg: CommandsAPI, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            CommandsAPI::ClientLogin(username, client_thread) => {
                let id = self.rng.gen::<usize>();
                
                // Gemmer klienten og sender dens id til thread
                self.clients.insert(id, (username.clone(), client_thread.clone()));
                client_thread.do_send(client_thread::api::IdentifierAPI::Set(id));

                // fortæller alle hvor mange spillere logget på!
                self.broadcast_active_players();

                // Tjek om den er med i et spil og fortæl klienten
                if self.is_player_in_game(&username) {
                    client_thread.do_send(client_thread::api::GameAPI::SetInGame(true));
                }
            },
            CommandsAPI::RemoveClient(client_id) => {
                self.clients.remove(&client_id);
            },
            CommandsAPI::RemoveEngine(engine_name) => {
                self.engines.remove(&engine_name);
            },
            CommandsAPI::RemoveGame(game_id) => {
                self.games.remove(&game_id);
            },
        };

        true
    }
}


impl<M> Handler<ToClientBrowserAPI<M>> for ServerThread where M: Serialize + std::marker::Send + std::fmt::Debug + Clone + 'static {
    type Result = bool;

    fn handle(&mut self, msg: ToClientBrowserAPI<M>, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            ToClientBrowserAPI::MessageToClientID(client_id, msg) => self.send_to_client_browser(client_id, msg),
            ToClientBrowserAPI::MessageToUsername(client_username, msg) => self.send_to_clients_browser(client_username, msg),
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
                    client_thread.do_send(client_thread::api::GameAPI::SetInGame(false));
                }
            },
            ClientCommandsAPI::NotifyYourTurn(game_id, username, fen_string) => {
                for (_, (client_username, client_thread)) in &self.clients {
                    if client_username != &username { continue }
                    client_thread.do_send(client_thread::api::GameAPI::YourTurn(fen_string));
                    return true;
                }

                for (engine_name, engine_thread) in &self.engines {
                    if engine_name != &username { continue }

                    let max_time = Duration::from_millis(250);
                    let fen = fen_string.clone();
                    // engine_thread.send(EngineThreadAPI::Search(fen_string.clone(), max_time));

                    engine_thread.send(EngineThreadAPI::Search(fen, max_time))
                        .into_actor(self)
                        .then(move |res, server_thread, server_thread_ctx| {
                            match res {
                                Ok(chess_move) => {
                                    let game = server_thread.games.get(&game_id).unwrap();

                                    game.2.do_send(game_thread::api::CommandsAPI::PlayMove(chess_move));
                                }
                                _ => server_thread_ctx.stop(),
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                }
            }
        };

        true
    }
}


impl Handler<GameCommandsAPI> for ServerThread {
    type Result = bool;

    fn handle(&mut self, msg: GameCommandsAPI, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            GameCommandsAPI::NewGame(me_username, opponent_username, time_format) => {
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

                    client_thread.do_send(client_thread::api::GameAPI::SetInGame(true));
                }

                true
            },
            GameCommandsAPI::GetBots(id_to_answer) => {
                let mut engine_names = Vec::new();

                for (engine_name, _) in &self.engines {
                    engine_names.push(engine_name.clone());
                }

                self.send_to_client_browser(id_to_answer, OutgoingWsMsg::content("engines", engine_names));

                true
            },
            GameCommandsAPI::RequestGameState(id_to_answer, username) => {
                let game = self
                    .games
                    .iter()
                    .find(|(_, (p1, p2, _))| {
                        *p1 == username || *p2 == username
                    });

                if let Some((_, (_, _, game_thread))) = game {
                    game_thread.do_send(game_thread::api::CommandsAPI::RequestGameState(id_to_answer));
                }

                true
            },
            GameCommandsAPI::PlayMove(username, chess_move) => {
                let game = self
                    .games
                    .iter()
                    .find(|(_, (p1, p2, _))| {
                        *p1 == username || *p2 == username
                    });

                if let Some((_, (_, _, game_thread))) = game {
                    game_thread.do_send(game_thread::api::CommandsAPI::PlayMove(chess_move));
                }

                true
            }
        }
    }
}


