use crate::{err_json, from_json, info, threads::{client::{ClientConn, Output}, server::ServerApi}, Content, Invite, MyError};

impl ClientConn {
    pub async fn invite(&mut self) -> anyhow::Result<Output> {
        let invite: Content<Invite> = from_json!(self);
  
        // hvis brugernavnet endnu ikke er sat...
        if self.username.is_none() { return err_json!(InviteFailed) }

        let from = self.username.clone().unwrap();
        let to = invite.content.to;

        self.server_api.send(ServerApi::Invite(from, to))?;

        Ok(Output::InviteSent)
    
    }


    pub async fn my_invites(&mut self) -> anyhow::Result<Output> {
        // hvis brugernavnet endnu ikke er sat...
        if self.username.is_none() { return Err(MyError::NotLoggedIn.into()) }

        // TODO: Find ud af hvilke invites brugeren har modtaget og sendt!



        // Ok(Output::TBA)
        err_json!(Idk)
    }
}
