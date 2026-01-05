use serde::Serialize;

use crate::TopicContent;


#[derive(Serialize)]
struct InviteOverview(Vec<(u64, String, String)>);


impl super::Server {
    /// Finder alle invitationer som en bruger er tilknyttet og sender dem til brugeren
    pub fn invite_overview(&self, user: super::Username) {
        let mut list: InviteOverview = InviteOverview(Vec::new());

        for (id, (from, to)) in &self.invites {
            if from == user.value() || to == user.value() {
                list.0.push((id.clone(), from.clone(), to.clone()));
            }
        }

        let msg = TopicContent {
            topic: "invite_overview".into(),
            result: true,
            content: list,
        };

        
        self.send_to_user(user.value(), msg);
    }
}
