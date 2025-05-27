use super::*;



#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub enum SocketAPI<M> where M: Serialize + std::marker::Send {
    /// En event som når modtages sender en besked direkte til klienten!
    SendDirectMessage(M),
}


#[derive(Message)]
#[rtype(result="bool")]
pub enum SocketSessionAPI {
    UpdateGameAddr(Addr<Game>),
    RemoveGameAddr,
}