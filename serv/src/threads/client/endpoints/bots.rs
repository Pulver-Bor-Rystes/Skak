use std::sync::mpsc;

use crate::{err_json, info, threads::{client::{ClientConn, Output}, server::{FetchApi, Answer, ServerApi, Username}}, MyError};


impl ClientConn {
    pub async fn query_engines(&self) -> anyhow::Result<Output> {
        if let Answer::List(names) = self.fetch_data(FetchApi::BotNames) {
            Ok(Output::List(names))
        }
        else {
            err_json!(Idk)
        }
    }



    pub async fn query_active_users(&self) -> anyhow::Result<Output> {
        if let Answer::List(names) = self.fetch_data(FetchApi::ActiveUsers) {
            Ok(Output::List(names))
        }
        else {
            err_json!(Idk)
        }
    }
    
}
