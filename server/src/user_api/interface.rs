use crate::communication::server::UpdateSessionData;
use crate::communication::session::SessionContext;
use crate::communication::std_format_msgs::{WrappedContent, WrappedResult};
use serde_json::Error as JsonError;

use super::auth;
use super::auth::LoginPayload;

pub fn handle(ctx: &mut SessionContext) -> Option<()> {
    let res = match ctx.topic.as_str() {
        "login" => handle_login(ctx),
        "signup" => handle_signup(ctx),
        _ => return None,
    };

    match res {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

fn handle_login(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: WrappedContent<LoginPayload> = serde_json::from_str(&ctx.msg)?;

    let username = msg.content.username.clone();
    match auth::login(msg.content) {
        Ok(success) => {
            ctx.srv
                .do_send(UpdateSessionData::LoggedIn(ctx.client_id, username));
            ctx.client
                .text(WrappedResult::content(&ctx.topic, success).serialize());
        }
        Err(err) => ctx
            .client
            .text(WrappedResult::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}

fn handle_signup(ctx: &mut SessionContext) -> Result<(), serde_json::Error> {
    let msg: WrappedContent<LoginPayload> = serde_json::from_str(&ctx.msg)?;

    match auth::signup(msg.content) {
        Ok(success) => {
            ctx.client
                .text(WrappedResult::content(&ctx.topic, success).serialize());
        }
        Err(err) => ctx
            .client
            .text(WrappedResult::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}
