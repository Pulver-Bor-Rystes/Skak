use engine_actor::EngineAPI;

use super::*;


impl Handler<GameAPI> for Server {
    type Result = bool;

    fn handle(&mut self, msg: GameAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            GameAPI::YourTurn(game_id, username, fen, time_left) => {
                let user = self
                    .clients
                    .iter()
                    .find(|(_id, sesh_data)| sesh_data.username == Some(username.clone()));

                if user.is_some() {
                    let id = user.unwrap().0;
                    let last_move: Option<String> = None;

                    self.deploy_msg(
                        vec![id.to_owned()],
                        OutgoingWsMsg::content("your turn", last_move),
                    );

                    return true;
                }

                let engine = self
                    .engines
                    .iter()
                    .find(|(engine_name, _addr)| engine_name == &&username);

                if engine.is_some() {
                    let (_engine_name, addr) = engine.unwrap();
                    addr.send(EngineAPI::Search(fen, time_left))
                        .into_actor(self)
                        .then(move |res, act, ctx| {
                            match res {
                                Ok(chess_move) => {
                                    let game = act.games.get(&game_id);
                                    match game {
                                        Some(game) => {
                                            game.addr.do_send(game_actor::API::Move(chess_move))
                                        }
                                        None => {}
                                    }
                                }
                                _ => ctx.stop(),
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                }
            }
        }
        true
    }
}