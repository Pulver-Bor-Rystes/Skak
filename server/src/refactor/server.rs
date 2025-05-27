// use super::{engine_actor::{self, Engine}, };
use super::*;

pub mod server_actor;
pub mod user_related_api;
pub mod game_related_api;


// API's


#[derive(Message)]
#[rtype(result = "bool")]
pub enum ServerUserAPI {
    /// String = Username
    GetRunningGame(String),
    RequestAvailableBots(usize),
    /// String = Username
    RequestGameState(String),
    /// Parameter oversigt:
    /// 1. Spillerens brugernavn
    /// 2. Ønsket modstander som brugernavn
    /// 3. Tidsformat
    NewGame(String, String, TimeFormat),
    SetGameAddr(String, Addr<Game>),
}




#[derive(Message)]
#[rtype(result = "bool")]
pub enum ServerGameAPI {
    /// Fortæller brugeren eller engine at det er deres tur¨
    /// 
    /// 1. game id
    /// 2. username
    /// 3. fen string
    /// 4. hvor meget tid spilleren har tilbage
    YourTurn(usize, String, String, Duration),
}






/// En struct der holder styr på de forskellige klienter og deres actors adresser
struct SessionData {
    addr: Addr<SocketSession>,
    username: Option<String>,
}

impl SessionData {
    fn new(addr: Addr<SocketSession>) -> Self {
        SessionData {
            addr,
            username: None,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.username.is_some()
    }
}

struct GameData {
    addr: Addr<Game>,
    p1: String,
    p2: String,
}

impl GameData {
    fn has_player(&self, username: &str) -> bool {
        &self.p1 == username || &self.p2 == username
    }
}



/// Et event andre `Actor`'s kan bruge til at sende en besked til en eller flere klienter
/// 
/// ### Vigtigt: String = Brugernavn
#[allow(dead_code)]
#[derive(Message, Debug)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub enum SendMessage<M: Serialize> {
    Broadcast(M),
    /// # String = Brugernavn
    /// 
    /// På den måde kan en bruger være logget ind flere steder og stadig få alle updates
    To(Vec<String>, M),
}

impl<M> Handler<SendMessage<M>> for Server
where
    M: Serialize + std::clone::Clone + std::marker::Send + 'static + std::fmt::Debug,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: SendMessage<M>, _ctx: &mut Context<Self>) -> Self::Result {
        // println!("Recieved msg: {:?}", msg);

        // få fat i klienternes addr og bed dem sende en besked!
        use SendMessage as Sm;
        match msg {
            Sm::Broadcast(msg) => {
                // find alle id'er
                let mut list_of_ids = vec![];
                for id in self.clients.keys() {
                    list_of_ids.push(id.clone());
                }

                // send til alle id'er / klienter
                self.deploy_msg(list_of_ids, msg.clone());
            }
            Sm::To(targets, msg) => {
                let mut id_list = vec![];

                self.clients.iter().for_each(|(id, sesh_data)| {
                    if sesh_data.username.is_none() {
                        return;
                    }

                    if !targets.contains(&sesh_data.username.as_ref().unwrap()) {
                        return;
                    }

                    id_list.push(id.clone());
                });

                self.deploy_msg(id_list, msg);
            }
        }

        Ok(true)
    }
}

/// Et event så en `session` kan opdatere sig selv.
///
/// `Connect`, `Disconnect` eller `LoggedIn(username: String)`
/// Når en `session` først oprettets skal den blot registreres.
/// Når en session bliver valideret ved at en bruger logger ind,
/// sender sessionen en opdatering med det givne brugernavn som
/// unikt kan identificere en session.
///
/// Det tillader på sigt at andre sessions kan sende beskeder til
/// en bruger uden at kende brugerens session id!
///
/// Returnerer `id: usize`, som skal bruges når sessionen engang slutter.
#[derive(Message, Debug)]
#[rtype(result = "Option<usize>")]
pub enum UpdateSessionData {
    Connect(Addr<SocketSession>),
    Disconnect(usize),
    LoggedIn(usize, String),
}

impl Handler<UpdateSessionData> for Server {
    type Result = Option<usize>;

    fn handle(&mut self, msg: UpdateSessionData, _ctx: &mut Context<Self>) -> Self::Result {
        use UpdateSessionData as Usd;
        match msg {
            Usd::Connect(sess_addr) => {
                // Gemmer klienten, så vi altid kan kommunikere til den
                let id = self.rng.gen::<usize>();
                let client = SessionData::new(sess_addr);
                self.clients.insert(id, client);

                return Some(id);
            }
            Usd::Disconnect(id) => {
                if let Some(client) = self.clients.remove(&id) {
                    if client.is_logged_in() {
                        self.broadcast_active_players();
                    }
                }
            }
            Usd::LoggedIn(id, username) => {
                if let Some(mut client) = self.clients.get_mut(&id) {
                    client.username = Some(username);

                    self.broadcast_active_players();
                }
            }
        }

        None
    }
}





// TODO: Fjern koden nedenunder og tilføj samme funktion direkte ind i game_api'en

#[derive(Message)]
#[rtype(result = "bool")]
pub enum Cleanup {
    Game(usize), // fjerner et spil via dets id!
}

impl Handler<Cleanup> for Server {
    type Result = bool;

    fn handle(&mut self, msg: Cleanup, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Cleanup::Game(id) => {
                let _ = self.games.remove(&id);
            }
        }

        true
    }
}
