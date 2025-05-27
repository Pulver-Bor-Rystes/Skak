use super::*;



/// En context så når en api endpoint skal svare tilbage, så kan de bede serveren om at gøre noget eller de
pub struct SessionContext<'a> {
    pub topic: String,
    pub msg: String,
    pub session: &'a mut SocketSession,
    // pub srv: Addr<Server>,
    pub socket: &'a mut WebsocketContext<SocketSession>,
    // pub client_id: usize,
    // pub client_username: String,
}

impl<'a> SessionContext<'a> {
    pub fn is_logged_in(&self) -> bool {
        self.session.username.is_some()
    }
}





pub fn test(ctx: &mut SessionContext) -> Result<(), JsonError> {
    ctx.socket.text(OutgoingWsMsg::content("another_test", "fuck yeah").serialize());

    Ok(())
}



pub fn play_move(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content::PlayMove> = serde_json::from_str(&ctx.msg)?;
    
    println!("play_move: {}", msg.content.chess_move);

    // ctx.session.game_addr.do_send(game_actor::API::Move(msg.content.chess_move));
    ctx.session.game_addr
        .as_mut()
        .unwrap()
        .do_send(game_actor::API::Move(msg.content.chess_move));

    Ok(())
}


pub fn login(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content::Login> = serde_json::from_str(&ctx.msg)?;

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

            ctx.session.server_addr.do_send(ServerUserAPI::GetRunningGame(username.clone()));
            ctx.session.username = Some(username);
        }
        Err(err) => ctx
            .socket
            .text(OutgoingWsMsg::error(&ctx.topic, err).serialize()),
    };

    Ok(())
}


pub fn signup(ctx: &mut SessionContext) -> Result<(), JsonError> {
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



pub fn getbots(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let _msg: IncomingWsTopic = serde_json::from_str(&ctx.msg)?;
    let id: usize = ctx.session.id;

    ctx.session.server_addr.do_send(ServerUserAPI::RequestAvailableBots(id));



    // get engines
    // send to client



    Ok(())
}



pub fn newgame(ctx: &mut SessionContext) -> Result<(), JsonError> {
    let msg: IncomingWsMsg<content_templates::NewGame> = serde_json::from_str(&ctx.msg)?;

    let format = TimeFormat::from(&msg.content.timeformat);

    let username = ctx.session.username.as_ref().unwrap().clone();
    let opponent = msg.content.username;

    ctx.session
        .server_addr
        .do_send(ServerUserAPI::NewGame(username, opponent, format.clone()));

    Ok(())
}



pub fn getstate(ctx: &mut SessionContext) -> Result<(), JsonError> {    
    let username = ctx.session.username.as_ref().unwrap().clone();

    // sig til serveren at vi gerne vil bede om en opdatering fra det spil vi er en del af!
    ctx.session
        .server_addr
        .do_send(ServerUserAPI::RequestGameState(username));

    Ok(())
}
