use std::{future::Future, pin::Pin, time::Instant};
use actix::Addr;
use actix_web_actors::ws::WebsocketContext;
use sqlx::{MySql, Pool};
use crate::{client_thread::ClientThread, error, info, server_thread::ServerThread, std_format_msgs::{json_templates, WsIn, WsOut, WsTopic}, warn};




#[derive(Debug)]
enum RequestRequirement {
    LoggedIn,
    InGame,
}


macro_rules! fn_name {
    ($f:expr) => {{
        fn f() {}
        let name = std::any::type_name_of_val(&$f);
        name.rsplit("::").next().unwrap()
    }};
}


impl ClientThread {
    pub fn new(server_addr: Addr<ServerThread>, sql_conn: Pool<MySql>) -> Self {
        Self {
            server_addr,
            sql_conn,
            id: None,
            username: None,
            hb: Instant::now(),
            in_game: false,
        }
    }


    async fn signup(&mut self, topic: &str, data: &str, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        assert_eq!(topic, "signup");
        
        let msg: WsIn<json_templates::Login> = serde_json::from_str(data)?;

       
        Ok(())
    }

    async fn login(&mut self, topic: &str, data: &str, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        assert_eq!(topic, "login");



        Ok(())
    }

    
    pub async fn client_endpoint(&mut self, topic: WsTopic, data: &str, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        self.login(&topic.topic, data, ctx).await?;
        self.signup(&topic.topic, data, ctx).await?;

        
        
        Ok(())
    }

    pub async fn nclient_endpoint(topic: WsTopic, data: String, ctx: &mut WebsocketContext<ClientThread>) -> Result<(), serde_json::Error> {
        // self.login(&topic.topic, data, ctx).await?;
        // self.signup(&topic.topic, data, ctx).await?;

        
        
        Ok(())
    }



    // private functions
    fn check_requirement(&self, requirement: &RequestRequirement) -> bool {
        use RequestRequirement::*;
        
        match requirement {
            LoggedIn => self.username.is_some() && self.id.is_some(),
            InGame => self.in_game,
        }
    }
}
