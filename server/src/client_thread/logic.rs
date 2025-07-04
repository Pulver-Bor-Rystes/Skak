use std::time::Instant;
use actix::Addr;
use actix_web_actors::ws::WebsocketContext;
use crate::{auth::{self, load}, error, game_thread::types::TimeFormat, info, server_thread::{self, ServerThread}, std_format_msgs::{content_templates, IncomingWsMsg, OutgoingWsMsg}};
use super::ClientThread;

#[derive(Debug)]
enum RequestRequirement {
    LoggedIn,
    InGame,
}


pub struct Request {
    topic: String,
    requires: Vec<RequestRequirement>,
    handler: fn(&mut ClientThread, &str, &IncomingWsMsg, &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error>,
}


impl ClientThread {
    pub fn new(server_addr: Addr<ServerThread>) -> Self {
        Self {
            addr: None, // Bliver sat så snart context starter
            server_addr,
            id: None,
            username: None,
            hb: Instant::now(),
            in_game: false,
        }
    }

    
    pub fn client_endpoint(&mut self, original_text: String, payload: IncomingWsMsg, ctx: &mut WebsocketContext<ClientThread>) -> bool {
        use RequestRequirement::*;


        let requests: Vec<Request> = vec![
            Request { topic: "login".into(), requires: [].into(), handler: ClientThread::login },
            Request { topic: "signup".into(), requires: [].into(), handler: ClientThread::signup },
            Request { topic: "newgame".into(), requires: [LoggedIn].into(), handler: ClientThread::new_game },
            Request { topic: "getbots".into(), requires: [LoggedIn].into(), handler: ClientThread::get_bots },
            Request { topic: "getstate".into(), requires: [InGame].into(), handler: ClientThread::get_state },
            Request { topic: "play_move".into(), requires: [InGame].into(), handler: ClientThread::play_move },
            Request { topic: "whats_my_rating".into(), requires: [LoggedIn].into(), handler: ClientThread::my_rating },
        ];

        let mut was_handled = false;

        for request in &requests {
            if self.addr.is_none() { continue }
            if payload.topic != request.topic { continue }

            let mut reqs_not_met = Vec::new();

            for requirement in &request.requires {
                if !self.check_requirement(requirement) { 
                    reqs_not_met.push(requirement);
                }
            }


            if reqs_not_met.len() > 0 {
                let err_msg = format!("Socket req: '{}' did not meet requirement(s): {reqs_not_met:?}", payload.topic);
                
                ctx.text(OutgoingWsMsg::error("game:fen_state", err_msg.clone()).serialize());
                error!("{err_msg}");
                was_handled = true;
                continue;
            }



            // go ahead
            match (request.handler)(self, &original_text, &payload, ctx) {
                Ok(_) => {
                    info!("Socket req: '{}'", request.topic);

                    was_handled = true;
                    break;
                },
                Err(error) => info!("[ERR: 318]\n\nRequest: {:?}\n\nError:{:?}", payload, error),
            }
        }

        was_handled
    }



    // private functions
    fn check_requirement(&self, requirement: &RequestRequirement) -> bool {
        use RequestRequirement::*;
        
        match requirement {
            LoggedIn => self.username.is_some() && self.id.is_some(),
            InGame => self.in_game,
        }
    }


    fn login(&mut self, original_text: &str, payload: &IncomingWsMsg, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        let msg: IncomingWsMsg<content_templates::Login> = serde_json::from_str(original_text)?;

        let username = msg.content.username.clone();
        match auth::login(msg.content) {
            Ok(success) => {
                self.username = Some(username.clone());
                self.server_addr.do_send(server_thread::api::CommandsAPI::ClientLogin(username, self.addr.clone().unwrap()));
                ctx.text(OutgoingWsMsg::content(&payload.topic, success).serialize());
            }
            Err(err) => ctx.text(OutgoingWsMsg::error(&payload.topic, err).serialize()),
        };

        Ok(())
    }


    fn signup(&mut self, original_text: &str, _payload: &IncomingWsMsg, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        let msg: IncomingWsMsg<content_templates::Login> = serde_json::from_str(original_text)?;

        match auth::signup(msg.content) {
            Ok(cookie) => {
                ctx.text(OutgoingWsMsg::content("signup", cookie).serialize());
            }
            Err(err) => {
                ctx.text(OutgoingWsMsg::error("signup", err).serialize());
            }
        }

        Ok(())
    }



    fn new_game(&mut self, original_text: &str, _payload: &IncomingWsMsg, _ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        let msg: IncomingWsMsg<content_templates::NewGame> = serde_json::from_str(original_text)?;

        let me = self.username.clone().unwrap();
        let opponent = msg.content.username;
        let time_format = TimeFormat::from(&msg.content.timeformat);

        self.server_addr.do_send(server_thread::api::GameCommandsAPI::NewGame(me, opponent, time_format));

        Ok(())
    }


    fn get_state(&mut self, _original_text: &str, _payload: &IncomingWsMsg, _ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        use server_thread::api::GameCommandsAPI::*;

        let id = self.id.unwrap();
        let username = self.username.clone().unwrap();
        
        self.server_addr.do_send(RequestGameState(id, username));

        Ok(())
    }
    
    

    fn get_bots(&mut self, _original_text: &str, _payload: &IncomingWsMsg, _ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        self.server_addr.do_send(server_thread::api::GameCommandsAPI::GetBots(self.id.unwrap()));
        Ok(())
    }

    fn play_move(&mut self, original_text: &str, _payload: &IncomingWsMsg, _ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        let msg: IncomingWsMsg<content_templates::PlayMove> = serde_json::from_str(original_text)?;
        
        self.server_addr.do_send(server_thread::api::GameCommandsAPI::PlayMove(self.username.clone().unwrap(), msg.content.chess_move));
        Ok(())
    }


    fn my_rating(&mut self, _original_text: &str, _payload: &IncomingWsMsg, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {        
        let username = self.username.clone().unwrap();
        let users = load();

        let user = users.list.get(&username);

        if let Some(user) = user {
            ctx.text(OutgoingWsMsg::content("rating", user.rating).serialize());
        }
        else {
            panic!("[619] Noget mærkeligt gik galt")
        }



        Ok(())
    }
}