use std::collections::HashMap;

use actix::prelude::*;
use serde::Serialize;

use super::session::{DeployMessage, Session};
use super::std::WrappedMsg as std_msg;
use rand::{self, rngs::ThreadRng, Rng};

struct Client {
    addr: Addr<Session>,
    username: Option<String>,
}

impl Client {
    fn new(addr: Addr<Session>) -> Self {
        Client {
            addr,
            username: None,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.username.is_some()
    }
}

pub struct Server {
    /// En liste over alle forbindelser uanset om de logget ind eller ej!
    clients: HashMap<usize, Client>,

    rng: ThreadRng,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn deploy_msg<M>(&mut self, id: &usize, msg: M)
    where
        M: Serialize + std::marker::Send + 'static + std::fmt::Debug,
    {
        let session = self.clients.get_mut(id);

        let msg: DeployMessage<M> = DeployMessage::IntoJson(msg);

        match session {
            Some(client) => client.addr.do_send(msg),
            _ => {}
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Ready to handle ws sessions!");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Server stopped");
    }
}

/// Et event andre `Actor`'s kan bruge til at sende en besked til en eller flere klienter
#[allow(dead_code)]
#[derive(Message, Debug)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub enum SendMessage<M: Serialize + std::fmt::Debug> {
    Broadcast(M),
    To(String, M),
}

impl<M> Handler<SendMessage<M>> for Server
where
    M: Serialize + std::fmt::Debug,
{
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: SendMessage<M>, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Recieved msg: {:?}", msg);

        // få fat i klienternes addr og bed dem sende en besked!
        match msg {
            SendMessage::Broadcast(_msg) => {}
            SendMessage::To(_target, _msg) => {}
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
    Connect(Addr<Session>),
    Disconnect(usize),
    LoggedIn(usize, String),
}

#[derive(Serialize, Debug)]
enum TestVal {
    test1(usize),
    test2(usize),
}
impl Handler<UpdateSessionData> for Server {
    type Result = Option<usize>;

    fn handle(&mut self, msg: UpdateSessionData, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            UpdateSessionData::Connect(sess_addr) => {
                let id = self.rng.gen::<usize>();
                println!("Allocating id: {}", id);

                let client = Client::new(sess_addr);
                self.clients.insert(id, client);

                // Send en test besked til klienten når socket forbindelsen bliver oprettet!
                let msg = std_msg::payload("test", TestVal::test1(123));
                self.deploy_msg(&id, msg);

                return Some(id);
            }
            UpdateSessionData::Disconnect(id) => {
                if let Some(client) = self.clients.remove(&id) {
                    if client.is_logged_in() {
                        println!("bye bye {}", client.username.unwrap())
                    }
                }
            }
            UpdateSessionData::LoggedIn(id, username) => {
                if let Some(mut client) = self.clients.get_mut(&id) {
                    client.username = Some(username);
                }
            }
        }

        None
    }
}

// struct IncomingMessage
