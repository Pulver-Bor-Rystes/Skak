use serde::Deserialize;

use crate::{err_json, from_json, info, threads::{client::{ClientConn, Output}, server::{ServerApi, Username}}, Auth, Content, MyError};



#[derive(Deserialize)]
struct CookieAuth {
    username: String,
    cookie: String,
}


struct Cookie {
    hash: String,
}


impl ClientConn {
    pub async fn login(&mut self) -> anyhow::Result<Output> {
        let auth: Content<Auth> = from_json!(self);

        let db_psw = sqlx::query!(
            "select hash from Password where owner = ? limit 1;",
            auth.content.username
        )
        .fetch_one(&self.db)
        .await?;

        if pwhash::bcrypt::verify(auth.content.password.clone(), &db_psw.hash) {
            let cookie = "random_cookie_string".to_string(); // TODO lav en tilfældig kode her
            let cookie_hash = pwhash::bcrypt::hash(&cookie)?;

            sqlx::query("insert into Cookie (hash, owner) values (?, ?)")
                .bind(cookie_hash.clone())
                .bind(auth.content.username.clone())
                .execute(&self.db)
                .await?;

            // gemmer brugernavnet
            self.username = Some(auth.content.username);

            self.server_api.send(ServerApi::ClientLogin(
                self.id,
                Username::from(self.username.as_ref().unwrap()),
                self.api.clone(),
            ))?;
            
            return Ok(Output::Cookie(cookie));
        }

        err_json!(WrongPassword)
    }

    pub async fn login_with_cookie(&mut self) -> anyhow::Result<Output> {
        let auth: Content<CookieAuth> = from_json!(self);

        let db_cookies: Vec<Cookie> = sqlx::query_as!(Cookie, "select hash from Cookie where lower(owner) = lower(?)", auth.content.username)
            .fetch_all(&self.db)
            .await?;

        

        for cookie in db_cookies {
            if pwhash::bcrypt::verify(auth.content.cookie.clone(), &cookie.hash.clone()) {
                self.username = Some(auth.content.username);

                info!("   \n --- 1. {:?}", self.id);

                self.server_api.send(ServerApi::ClientLogin(
                    self.id,
                    Username::from(self.username.as_ref().expect("kunne ikke unwrap username")),
                    self.api.clone(),
                ))?;

                return Ok(Output::AllGood);
            }
        }

        err_json!(WrongCookie)
       
    }
}
