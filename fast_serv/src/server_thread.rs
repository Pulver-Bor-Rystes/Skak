mod game;

use crate::prelude::*;
use crate::server_thread::game::Game;
use crate::{
    client_thread::ClientApi,
    engine_thread::{Bot, EngineApi},
    parsing::{self, Json, Message, RequestResp},
    types::*,
};
use log::{info, warn};
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

pub struct Server {
    rx: Receiver<ServerApi>,
    // tx: Sender<ServerApi>,
    clients: HashMap<Username, Vec<ClientEntity>>,
    engines: HashMap<Username, Sender<EngineApi>>,
    /// FromTo_Hash, (From, To)
    invites: HashMap<u64, (Username, Username)>,
    game_invites: HashMap<u64, (Username, Username)>,
    games: HashMap<u64, Game>,
}

struct ClientEntity {
    api: Sender<ClientApi>,
    id: Uuid,
}

pub enum ServerApi {
    RegisterClientLogin(Uuid, Username, Sender<ClientApi>),
    ClientDestroy(Username, Uuid),
    /// From, To
    Invite(Username, Username),
    // Game stuff
    /// **game_hash**: u64, **move_name**: String
    GameMove(Uuid, u64, String),
    GameInvite(Username, Username),
    /// Input til GameAcceptInvite:
    ///
    /// (**Me**: Username, **game_hash**: u64)
    GameAcceptInvite(Username, u64),

    QueryEngines(Uuid),
    QueryActiveUsers(Uuid),
    QueryInviteState,
    GameCancelInvite(u64, Username),
}

impl Server {
    pub fn new() -> Sender<ServerApi> {
        let (tx, rx) = mpsc::channel::<ServerApi>();

        let mut server = Server {
            rx,
            clients: HashMap::new(),
            engines: HashMap::new(),
            invites: HashMap::new(),
            game_invites: HashMap::new(),
            games: HashMap::new(),
        };

        std::thread::spawn(move || server.server_thread());

        tx
    }

    /// sender en liste over alle forbundede brugere til alle klienter.
    fn update_whos_online(&self) {
        let mut list = Vec::new();

        for k in self.clients.keys() {
            list.push(k.clone());
        }

        for username in &list {
            let mut list = list.clone();
            list.retain(|u| u != username);

            let msg = parsing::Message {
                topic: "query:active_users".into(),
                result: true,
                content: Box::new(RequestResp::List(list)),
            };

            self.send_to_user(username, msg);
        }
    }

    /// Sender en JSON besked til alle klienter med det brugernavn
    fn send_to_user(&self, username: &Username, msg: impl Json) {
        if let Some(clients) = self.clients.get(username) {
            for c in clients {
                match c.api.send(ClientApi::SendJSON(msg.to_json())) {
                    Err(send_err) => {
                        warn!("Kunne ikke sende besked til user client {:?}", send_err)
                    }
                    Ok(_) => {}
                }
            }
        } else {
            info!("Brugeren findes ikke {username}");
        }
    }

    fn send_to_client<T: Json>(&self, uuid: &Uuid, msg: T) {
        for user in &self.clients {
            for client in user.1 {
                if client.id == *uuid {
                    match client.api.send(ClientApi::SendJSON(msg.to_json())) {
                        Ok(_) => {}
                        Err(send_err) => {
                            warn!("Kunne ikke sende besked til user client {:?}", send_err)
                        }
                    }
                }
            }
        }
    }

    /// Finder alle spil, som brugeren bør kende til og sender
    /// detaljerne til alle forbundne klienter
    fn broadcast_game_details(&self, username: &Username) {
        let games = self
            .games
            .iter()
            .filter(|(_, game)| game.does_player_need_to_know(username))
            .map(|(_, game)| game.clone())
            .collect::<Vec<Game>>();

        self.send_to_user(
            username,
            Message {
                topic: "game:games".to_string(),
                result: true,
                content: Box::new(games),
            },
        );
    }

    fn broadcast_game_update(&self, game_hash: &u64) {
        let Some(game) = self.games.get(game_hash) else {
            return;
        };

        for u in &game.players {
            self.send_to_user(
                u,
                Message {
                    topic: format!("game:{game_hash}:update"),
                    result: true,
                    content: Box::new(game.clone()),
                },
            );
        }
    }

    /// Finder alle invitationer som en bruger er tilknyttet og sender dem til brugeren
    fn broadcast_invite_overview(&self, user: &Username) {
        let mut friend_invite_list: Vec<(u64, Username, Username)> = Vec::new();
        let mut game_invite_list: Vec<(u64, Username, Username)> = Vec::new();

        // game invites
        for (invite_hash, (from, to)) in &self.game_invites {
            if from == user || to == user {
                game_invite_list.push((invite_hash.clone(), from.clone(), to.clone()));
            }
        }

        // regular friend invites
        for (invite_hash, (from, to)) in &self.invites {
            if from == user || to == user {
                friend_invite_list.push((invite_hash.clone(), from.clone(), to.clone()));
            }
        }

        let msg_game = Message {
            topic: "friend:invite_overview".into(),
            result: true,
            content: Box::new(RequestResp::Value(
                "friend_invite_overview".into(),
                friend_invite_list.prepare_json(),
            )),
        };

        let msg_friend = Message {
            topic: "game:invite_overview".into(),
            result: true,
            content: Box::new(RequestResp::Value(
                "game_invite_overview".into(),
                game_invite_list.prepare_json(),
            )),
        };

        self.send_to_user(&user, msg_game);
        self.send_to_user(&user, msg_friend);
    }

    fn hash_str(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    pub fn server_thread(&mut self) {
        info!("Server Thread is online");

        use ServerApi::*;

        loop {
            let msg = self.rx.try_recv();
            if let Ok(msg) = msg {
                match msg {
                    RegisterClientLogin(id, username, api) => {
                        let c = ClientEntity { api, id };

                        // hvis brugeren allerede er logget ind via en anden browser
                        if let Some(client) = self.clients.get_mut(&username) {
                            client.push(c);
                        } else {
                            // hvis brugeren ikke er logget ind andre steder
                            self.clients.insert(username.clone(), vec![c]);
                        }

                        self.update_whos_online();
                        self.broadcast_invite_overview(&username);
                        self.broadcast_game_details(&username);
                    }
                    Invite(from, to) => {
                        // laver et hash, som bruges til at identificere invitationen
                        let combined = format!("{from}, {to}");
                        let c_hash = Self::hash_str(&combined);

                        // hvis invi allerede eksisterer, stop op og fortæl browser
                        if self.invites.contains_key(&c_hash) {
                            self.send_to_user(
                                &from,
                                Message {
                                    topic: "invite".into(),
                                    result: false,
                                    content: Box::new(RequestResp::Value(
                                        "could_not_invite".into(),
                                        to.to_string(),
                                    )),
                                },
                            );

                            continue;
                        }

                        // gemmer invitationen
                        self.invites.insert(c_hash, (from.clone(), to.clone()));

                        // fortæller modtageren at de har fået en invitation.
                        // let msg = TopicContent {
                        //     topic: "invite_receive".into(),
                        //     result: true,
                        //     content: from,
                        // };

                        self.send_to_user(
                            &to,
                            Message {
                                topic: "invite".to_string(),
                                result: true,
                                content: Box::new(from.clone()),
                            },
                        );

                        let users = vec![Username::from(from), Username::from(to)];
                        for u in users {
                            self.broadcast_invite_overview(&u);
                        }
                    }
                    GameMove(client_uuid, game_hash, move_name) => {
                        // find spillet
                        let Some(game) = self.games.get_mut(&game_hash) else {
                            continue;
                        };

                        match game.play_move(&move_name) {
                            Ok(_) => {
                                info!("updating everybody");
                                self.broadcast_game_update(&game_hash);
                            }
                            Err(e) => {
                                self.send_to_client(
                                    &client_uuid,
                                    Message {
                                        topic: format!("game:{game_hash}:move"),
                                        result: false,
                                        content: Box::new(e),
                                    },
                                );
                            }
                        }
                    }
                    GameCancelInvite(hash, username) => {
                        let Some((from, to)) = self.game_invites.get(&hash) else {
                            continue;
                        };

                        if *from != username {
                            continue;
                        }

                        let users = vec![from.clone(), to.clone()];
                        self.game_invites.remove(&hash);

                        for u in users {
                            self.broadcast_invite_overview(&u);
                        }
                    }
                    GameAcceptInvite(username, game_hash) => {
                        // tjekker om invitation eksisterer
                        if let Some((from, to)) = self.game_invites.get(&game_hash) {
                            // hvis invitation er til spilleren som prøver at acceptere
                            if *to == username {
                                let from = from.clone();
                                let to = to.clone();

                                self.game_invites.remove(&game_hash);

                                self.games.insert(
                                    game_hash,
                                    Game::new(game_hash, [from.clone(), to.clone()]),
                                );

                                // opdater invitationer for klienterne
                                for username in [from, to] {
                                    self.broadcast_game_details(&username);
                                    self.broadcast_invite_overview(&username);
                                }
                            }
                        }
                    }
                    GameInvite(from, to) => {
                        // TODO: Tjek om de overhovedet er venner
                        // Tjek hvorvidt invitation allerede eksisterer.
                        let invi_from_sender = Self::hash_str(&format!("{from}, {to}"));
                        let invi_from_receiver = Self::hash_str(&format!("{to}, {from}"));

                        if self.game_invites.contains_key(&invi_from_sender) {
                            self.send_to_user(
                                &from,
                                Message {
                                    topic: "game:invite".to_string(),
                                    result: false,
                                    content: Box::new("Invitation already exists".to_string()),
                                },
                            );
                            continue;
                        }

                        // Vi bør acceptere invitationen!
                        if self.game_invites.contains_key(&invi_from_receiver) {
                            self.send_to_user(
                                &from,
                                Message {
                                    topic: "game:invite".to_string(),
                                    result: false,
                                    content: Box::new(
                                        "They already invited you, now the game begins".to_string(),
                                    ),
                                },
                            );
                            self.send_to_user(
                                &to,
                                Message {
                                    topic: "game:invite".to_string(),
                                    result: false,
                                    content: Box::new(
                                        "Opponent accepted your invitation".to_string(),
                                    ),
                                },
                            );
                            continue;
                        }

                        // Ingen af invitationerne eksisterer!
                        self.game_invites
                            .insert(invi_from_sender, (from.clone(), to.clone()));

                        // fortæller modtageren at invitationen er sendt
                        self.send_to_user(
                            &from,
                            Message {
                                topic: "game:invite".to_string(),
                                result: true,
                                content: Box::new(RequestResp::GameInviteSent(to.clone())),
                            },
                        );

                        // fortæller modtageren at invitationen er sendt
                        self.send_to_user(
                            &to,
                            Message {
                                topic: "game:invite".to_string(),
                                result: true,
                                content: Box::new(RequestResp::GameInviteReceived(from.clone())),
                            },
                        );

                        let users = vec![Username::from(from), Username::from(to)];
                        for u in users {
                            self.broadcast_invite_overview(&u);
                        }
                    }
                    QueryInviteState => {
                        dbg!(self.invites.clone());
                    }
                    ClientDestroy(username, id) => {
                        let mut remove_client_all_together = false;
                        if let Some(clients) = self.clients.get_mut(&username) {
                            clients.retain(|c| c.id != id);

                            remove_client_all_together = clients.is_empty();
                        }

                        if remove_client_all_together {
                            self.clients.remove(&username);
                            self.invites.retain(|_hash, (from, _to)| from != &username);
                        }

                        self.update_whos_online();
                    }
                    QueryEngines(uuid) => {
                        // find bots
                        let bots: Vec<Bot> = vec![
                            Bot {
                                name: "stockfish".into(),
                                rating: 3700,
                            },
                            Bot {
                                name: "sisyphus".into(),
                                rating: 1800,
                            },
                        ];
                        let msg = Message {
                            topic: "query:engines".into(),
                            result: true,
                            content: Box::new(bots),
                        };

                        self.send_to_client(&uuid, msg);
                    }
                    QueryActiveUsers(uuid) => {
                        let clients: Vec<Username> = self.clients.keys().cloned().collect();

                        let msg = Message {
                            topic: "query:active_users".into(),
                            result: true,
                            content: Box::new(clients),
                        };

                        self.send_to_client(&uuid, msg);
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
