use crate::actors::server::{self, UpdateSessionData};
use crate::actors::session::SessionContext;
use crate::std_format_msgs::{WrappedContent, WrappedResult};
use serde::de::Error;
use serde_json::Error as JsonError;

use super::auth;
use crate::std_format_msgs::content_templates;

pub fn handle(ctx: &mut SessionContext) -> Option<()> {
    let res = match ctx.topic.as_str() {
        "login" => handle_login(ctx),
        "signup" => handle_signup(ctx),
        "newgame" => handle_newgame(ctx),
        "getstate" => handle_getstate(ctx),
        _ => return None,
    };

    match res {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

fn handle_login(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: WrappedContent<content_templates::Login> = serde_json::from_str(&ctx.msg)?;

    let username = msg.content.username.clone();
    match auth::login(msg.content) {
        Ok(success) => {
            ctx.session.server_addr.do_send(UpdateSessionData::LoggedIn(
                ctx.session.id,
                username.clone(),
            ));
            ctx.socket
                .text(WrappedResult::content(&ctx.topic, success).serialize());

            ctx.session.username = Some(username);
        }
        Err(err) => ctx
            .socket
            .text(WrappedResult::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}

fn handle_signup(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: WrappedContent<content_templates::Login> = serde_json::from_str(&ctx.msg)?;

    match auth::signup(msg.content.clone()) {
        Ok(success) => {
            ctx.socket
                .text(WrappedResult::content(&ctx.topic, success).serialize());

            ctx.session.username = Some(msg.content.username);
        }
        Err(err) => ctx
            .socket
            .text(WrappedResult::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}

fn handle_newgame(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: WrappedContent<content_templates::Username> = serde_json::from_str(&ctx.msg)?;

    ctx.session.server_addr.do_send(server::API::NewGame(
        ctx.session.username.clone().unwrap_or("".to_string()),
        msg.content.username,
    ));

    Ok(())
}

fn handle_getstate(ctx: &mut SessionContext) -> Result<(), JsonError> {
    if ctx.session.username.is_none() {
        return Err(Error::custom("not logged in!"));
    }

    let username = ctx.session.username.as_ref().unwrap().clone();

    // sig til serveren at vi gerne vil bede om en opdatering fra det spil vi er en del af!
    ctx.session
        .server_addr
        .do_send(server::API::RequestGameState(username));

    Ok(())
}
