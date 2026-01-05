use crate::{err_json, error, from_json, threads::client::{ClientConn, Output}, Auth, Content, MyError};

impl ClientConn {
    pub async fn signup(&mut self) -> anyhow::Result<Output> {
        let content: Content<Auth> = from_json!(self);

        let hash = pwhash::bcrypt::hash(content.content.password)?;
    
        sqlx::raw_sql("start transaction;").execute(&self.db).await?;


        let user = sqlx::query("insert into User (username) values (?)")
            .bind(content.content.username.clone())
            .execute(&self.db)
            .await;

        let psw = sqlx::query("insert into Password (hash, owner) values (?, ?)")
            .bind(hash)
            .bind(content.content.username)
            .execute(&self.db)
            .await;


        if user.is_err() || psw.is_err() {
            error!("an error occured.\nrolling back...\nuser: {user:#?}, \npassword: {psw:#?}");
            sqlx::raw_sql("rollback").execute(&self.db).await?;
            err_json!(UnableToSignup)
        }
        else {
            sqlx::raw_sql("commit").execute(&self.db).await?;
            return Ok(Output::SignupSuccesful);
        }
    }
}
