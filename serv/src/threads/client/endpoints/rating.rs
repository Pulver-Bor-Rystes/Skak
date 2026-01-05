use crate::{MyError, err_json, threads::client::{ClientConn, Output}};

impl ClientConn {
    pub async fn whats_my_rating(&self) -> anyhow::Result<Output> {
        if let Some(username) = &self.username {
            let user = sqlx::query!("select rating from User where username = ?", username)
                .fetch_one(&self.db)
                .await?;

        
            Ok(Output::Rating(user.rating))
        }
        else {
            err_json!(NotLoggedIn)
        }
    }
}
