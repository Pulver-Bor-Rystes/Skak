use crate::{client_thread::{ClientAPI, ClientThread}, info};
use actix::prelude::*;
use sqlx::{MySql, Pool};
use std::{collections::HashMap, sync::mpsc::{Receiver, Sender}};

pub type Username = String;
pub type ID = usize;



#[derive(Message)]
#[rtype(result="()")]
pub enum ServerAPI {
    ClientNew(Addr<ClientThread>),
    /// usize: ID *tildelt af server*
    /// String: username
    ClientLogin(usize, String),
    RemoveClient(usize),
}


pub struct ServerThread {
    clients: HashMap<ID, (Option<Username>, Addr<ClientThread>)>,
    sql_conn: Pool<MySql>,
}



impl Actor for ServerThread {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        // self.start_bots(ctx).expect("Noget gik galt");
        info!("Ready to handle ws sessions!");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Server stopped");
    }
}


impl Handler<ServerAPI> for ServerThread {
    type Result = ();


    fn handle(&mut self, msg: ServerAPI, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ServerAPI::ClientNew(addr) => {
                let id = self.gen_client_id(0);
                addr.do_send(ClientAPI::SetID(id));
                self.clients.insert(id, (None, addr));
            },
            ServerAPI::ClientLogin(id, new_username) => {
                if let Some((username, _)) = self.clients.get_mut(&id) {
                    let _ = username.insert(new_username);
                }
            }
            ServerAPI::RemoveClient(id) => {
                self.clients.remove(&id);
            }
        }
    }
 
}


impl ServerThread {
    pub fn new(sql_pool: Pool<MySql>) -> Self {
        Self {
            clients: HashMap::new(),
            sql_conn: sql_pool,
        }
    }


    fn gen_client_id(&self, i: usize) -> usize {
        for (id, _) in &self.clients {
            if &i == id {
                return self.gen_client_id(i + 1);
            }
        }
        return i;
    }

    // pub fn start_bots(&mut self, ctx: &mut Context<ServerThread>) -> Result<(), std::io::Error> {
    //     let bots = std::fs::read_dir("./bots")?;

    //     for bot in bots {
    //         let path = bot?.path().into_os_string().into_string().unwrap();
    //         let name = path.split("/").last().unwrap_or("NAMING GONE WRONG"); // burde kun kunne gå galt, hvis programmet køres på en windows maskine

    //         match EngineThread::new(&path, ctx.address()) {
    //             Some(actor) => {
    //                 self.engines.insert(name.into(), actor.start());
    //             }
    //             None => error!("When trying to load {path} an error occured?"),
    //         }
    //     }

    //     return Ok(());
    // }
}
