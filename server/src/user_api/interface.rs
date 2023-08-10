use super::auth;
use super::auth::LoginPayload;
use super::types;
use crate::com::{MessageHandler, WSMessage};

pub fn handle(ctx: &mut impl MessageHandler, payload: &String) {
    let _ = handle_login(ctx, &payload);
    let _ = handle_signup(ctx, &payload);
}

fn handle_login(ctx: &mut impl MessageHandler, payload: &String) -> Result<(), serde_json::Error> {
    let message: WSMessage<LoginPayload> = serde_json::from_str(&payload)?;
    println!("login: {}", payload);
    if message.topic != "login" {
        return Ok(());
    }

    match auth::login(message.data) {
        Ok(loginsuccess) => match loginsuccess {
            types::LoginSuccess::Cookie(cookie) => {
                ctx.ok("login", cookie);
            }
            types::LoginSuccess::LoggedIn => {
                ctx.ok("login", "logged in");
            }
        },
        Err(loginerror) => ctx.error("login", loginerror),
    }

    Ok(())
}

fn handle_signup(ctx: &mut impl MessageHandler, payload: &String) -> Result<(), serde_json::Error> {
    let message: WSMessage<LoginPayload> = serde_json::from_str(&payload)?;
    if message.topic != "signup" {
        return Ok(());
    }

    match auth::signup(message.data) {
        Ok(cookie) => ctx.ok("signup", cookie),
        Err(signuperror) => ctx.error("login", signuperror),
    }

    Ok(())
}
