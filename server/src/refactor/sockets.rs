use super::*;


pub mod socket_actor;
pub mod socket_endpoint;
pub mod socket_api;




#[derive(Debug)]
pub struct SocketSession {
    pub server_addr: Addr<Server>,
    pub game_addr: Option<Addr<Game>>,
    /// Id bliver givet af serveren
    pub id: usize,
    pub hb: Instant,

    // data der skal registreres på socket'en
    pub username: Option<String>,
}




