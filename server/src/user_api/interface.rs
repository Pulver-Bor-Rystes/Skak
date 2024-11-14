use crate::actors::game::TimeFormat;
use crate::actors::server;
use crate::actors::session::SessionContext;
use crate::std_format_msgs::{IncomingWsMsg, OutgoingWsMsg};
use serde_json::Error as JsonError;

use super::auth;
use crate::std_format_msgs::content_templates;

pub fn handle(ctx: &mut SessionContext) -> Option<()> {
    let res;

    if ctx.is_logged_in() {
        res = match ctx.topic.as_str() {
            "newgame" => handle_newgame(ctx),
            "getstate" => handle_getstate(ctx),
            _ => return None,
        }
    } else {
        res = match ctx.topic.as_str() {
            "login" => handle_login(ctx),
            "signup" => handle_signup(ctx),
            _ => return None,
        };
    }

    match res {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

fn handle_login(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content_templates::Login> = serde_json::from_str(&ctx.msg)?;

    let username = msg.content.username.clone();
    match auth::login(msg.content) {
        Ok(success) => {
            ctx.session
                .server_addr
                .do_send(server::UpdateSessionData::LoggedIn(
                    ctx.session.id,
                    username.clone(),
                ));
            ctx.socket
                .text(OutgoingWsMsg::content(&ctx.topic, success).serialize());

            ctx.session.username = Some(username);
        }
        Err(err) => ctx
            .socket
            .text(OutgoingWsMsg::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}

fn handle_signup(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content_templates::Login> = serde_json::from_str(&ctx.msg)?;

    match auth::signup(msg.content.clone()) {
        Ok(success) => {
            ctx.socket
                .text(OutgoingWsMsg::content(&ctx.topic, success).serialize());

            ctx.session.username = Some(msg.content.username);
        }
        Err(err) => ctx
            .socket
            .text(OutgoingWsMsg::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}

fn handle_newgame(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content_templates::NewGame> = serde_json::from_str(&ctx.msg)?;

    let format = TimeFormat::from(&msg.content.timeformat);

    let username = ctx.session.username.as_ref().unwrap().clone();
    let opponent = msg.content.username;

    ctx.session
        .server_addr
        .do_send(server::UserAPI::NewGame(username, opponent, format.clone()));

    Ok(())
}

fn handle_getstate(ctx: &mut SessionContext) -> Result<(), JsonError> {
    println!("request: {:?}", ctx.topic);
    
    let username = ctx.session.username.as_ref().unwrap().clone();

    // sig til serveren at vi gerne vil bede om en opdatering fra det spil vi er en del af!
    ctx.session
        .server_addr
        .do_send(server::UserAPI::RequestGameState(username));

    Ok(())
}
