use super::*;


#[derive(Message)]
#[rtype(result = "bool")]
pub enum UserAPI {
    RequestAvailableBots(usize),
    /// String = Username
    RequestGameState(String),
    /// Parameter oversigt:
    /// 1. Spillerens brugernavn
    /// 2. Ønsket modstander som brugernavn
    /// 3. Tidsformat
    NewGame(String, String, TimeFormat),
}

impl Handler<UserAPI> for Server {
    type Result = bool;

    fn handle(&mut self, msg: UserAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserAPI::RequestAvailableBots(id) => {
                let mut engines = Vec::new();
                let keys = self.engines.keys();

                for e in keys.cloned() {
                    engines.push(e);
                }

                self.deploy_msg(vec![id], OutgoingWsMsg::content("engines", engines));
                
            }
            UserAPI::RequestGameState(username) => {
                let game = self.find_game(&username);

                if game.is_some() {
                    game.unwrap().addr.do_send(game::API::GetState(username));
                }
            }
            UserAPI::NewGame(p1, opponent, time_format) => {
                let id = self.rng.gen::<usize>();

                // er en af spillerene i gang med et spil?
                if self.find_game(&opponent).is_some() || self.find_game(&p1).is_some() {
                    return false;
                }

                let mut player2: Option<String> = None;

                // tjekker først om det er en engine vi vil spille imod!
                match self.engines.iter().find(|(name, _addr)| &&opponent == name) {
                    Some(_) => player2 = Some(opponent.clone()),
                    None => {}
                }

                // finder den første spiller der har det brugernavn
                match self
                    .clients
                    .iter()
                    .filter(|client| client.1.is_logged_in())
                    .find(|client| client.1.username.as_ref().unwrap() == &opponent)
                {
                    Some(_) => player2 = Some(opponent),
                    None => {}
                }

                if player2.is_none() {
                    return false;
                }

                let p2 = player2.unwrap();

                // start spillet
                let game = Game::new(&id, &ctx.address(), &p1, &p2, &time_format);

                // starter en actor og gemmer den :)
                let addr = game.start();
                self.games.insert(id, GameData { addr, p1, p2 });
            }
        }

        true
    }
}
