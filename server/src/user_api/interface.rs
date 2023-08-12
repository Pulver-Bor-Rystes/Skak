use super::auth;
use super::auth::LoginPayload;
use super::types;
use crate::com::WSMessage;
use crate::server::SocketContext;

pub fn handle(ctx: &mut SocketContext) {
    let _ = match ctx.topic.as_str() {
        "login" => handle_login(ctx),
        "signup" => handle_login(ctx),
        _ => Ok(()),
    };
}

fn handle_login(ctx: &mut SocketContext) -> Result<(), serde_json::Error> {
    let message: WSMessage<LoginPayload> = serde_json::from_str(&ctx.get_msg())?;
    println!("message: {:?}", message);

    match auth::login(message.data) {
        Ok(loginsuccess) => match loginsuccess {
            types::LoginSuccess::Cookie(cookie) => {
                ctx.ok(cookie);
            }
            types::LoginSuccess::LoggedIn => {
                ctx.ok("logged in");
            }
        },
        Err(loginerror) => ctx.error(loginerror),
    }

    Ok(())
}

fn handle_signup(ctx: &mut SocketContext) -> Result<(), serde_json::Error> {
    let message: WSMessage<LoginPayload> = serde_json::from_str(&ctx.get_msg())?;

    match auth::signup(message.data) {
        Ok(cookie) => ctx.ok(cookie),
        Err(signuperror) => ctx.error(signuperror),
    }

    Ok(())
}
