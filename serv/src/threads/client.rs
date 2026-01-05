use crate::{
    MyError, Topic, TopicContent, err_json, error, info,
    threads::server::{Answer, FetchApi, ServerApi, Username},
    to_json,
};
use actix_web::{
    HttpRequest, HttpResponse,
    web::{self, Data},
};
use actix_ws::{AggregatedMessage, Session};
use serde::Serialize;
use smol::stream::StreamExt;
use sqlx::{MySql, Pool};
use std::{
    sync::{
        Arc, RwLock,
        mpsc::{self, Sender},
    },
    time::Duration,
};

pub mod endpoints;
// pub mod new_client_connection;

pub struct ClientConn {
    id: usize,
    username: Option<String>,

    // meta stuff
    request: Option<String>,
    session: Session,
    db: Pool<MySql>,
    server_api: Sender<ServerApi>,
    api: Sender<ClientApi>,
    /// Køen bliver brugt til at gemme alle de beskeder, som serveren f.eks. gerne vil sende til browseren.
    queue: Vec<String>,
}

impl ClientConn {
    pub async fn distribute(&mut self, t: Topic) -> anyhow::Result<Output> {
        let r = match t.topic.as_str() {
            "login" => self.login().await?,
            "login_with_cookie" => self.login_with_cookie().await?,
            "signup" => self.signup().await?,
            "whats_my_rating" => self.whats_my_rating().await?,
            "query_engines" => self.query_engines().await?,
            "active_users" => self.query_active_users().await?,
            "invite_player" => self.invite().await?,
            "accept_invite" => todo!(""),
            _ => return err_json!(TopicNotHandled),
        };

        Ok(r)
    }

    /// Bruges til at hente data fra serveren.
    /// Der burde ikke kunne opstå fejl, så vi returnere bare et Answer
    /// og panikker, hvis ikke, så opdager jeg nok fejlen på et tidspunkt haha
    pub fn fetch_data(&self, api: FetchApi) -> Answer {
        let (tx, rx) = mpsc::channel::<Answer>();

        match self.server_api.send(ServerApi::FetchData(tx, api)) {
            Err(e) => panic!("{e}"),
            _ => {}
        }

        match rx.recv() {
            Ok(v) => return v,
            Err(e) => panic!("{e}"),
        }
    }

    /// Sender køen af JSON beskeder til klienten
    async fn send_queue(&mut self) {
        for msg in self.queue.clone() {
            self.session
                .text(msg)
                .await
                .expect("Kunne ikke sende en besked fra køen");
        }
        self.queue.clear();
    }

    /// håndterer scenariet, når en browser et ping
    async fn handle_ping_msg(&mut self, text: &str) {
        if text == "ping" {
            let pong_msg = to_json!(Topic {
                topic: "pong".into()
            });
            self.session.text(pong_msg).await.unwrap();
        }
    }

    // pub fn request(&self) -> String {
    //     let a = MyError
    //     self.request.as_ref().unwrap().clone()
    // }

    async fn handle_topic(&mut self, topic: Topic) {
        // forsøger at håndtere requesten!
        let handle_response = self.distribute(topic.clone()).await;

        match handle_response {
            Ok(output) => {
                if output == Output::Pending {
                    return;
                }

                let msg = TopicContent {
                    topic: topic.topic,
                    result: true,
                    content: output,
                };

                let json = to_json!(msg);

                self.session.text(json).await.expect("Noget gik galt");
            }
            Err(err) => {
                let msg = TopicContent {
                    topic: topic.topic,
                    result: false,
                    content: format!("{err}"),
                };

                let json = to_json!(msg);

                self.session.text(json).await.expect("Noget gik galt");
            }
        }

        self.request = None;
    }
}

pub enum ClientApi {
    SendJSON(String),
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Output {
    /// Bliver brugt, når en request bliver smidt over til
    /// serveren, som senere kan give besked til client
    /// at den skal sende noget til browseren
    /// kort sagt: request modtaget og bliver fortsat behandlet
    Pending,
    InviteSent,
    Cookie(String),
    AllGood,

    /// bliver brugt til lidt af hvert
    /// primært når vi henter aktive brugere og bots
    List(Vec<String>),

    SignupSuccesful,
    Rating(i32),
}

/// Hver gang en browser starter en websocket forbindelse til serveren
/// kører vi denne funktion.
/// Her starter vi to ting, som kører uafhængigt af hinanden.
/// 1. En thread, hvor alle api kald indefra *(altså fra serveren)* bliver håndteret
/// 2. En async runtime, hvor alt udefra bliver håndteret.
///
/// Vi bruger en async runtime, fordi vi gerne vil kunne lave sql queries via sqlx!
/// Vi deler alle oplysninger mellem de to, via en Arc<RwLock<ClientConn>>, som sørger for at vi tilfår informationen på en ordentlig måde!
///
pub async fn new_socket_conn(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Pool<MySql>>,
    server_api_tx: Data<Sender<ServerApi>>,
    latest_id: Data<Arc<RwLock<usize>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut lid = latest_id.write().unwrap();
    let id = *lid;
    *lid += 1;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    // api input output
    let (client_api_tx, client_api_rx) = mpsc::channel::<ClientApi>();

    // selve client conn objektet
    let client = Arc::new(RwLock::new(ClientConn {
        id,
        username: None,

        // meta stuff
        request: None,
        session,
        db: db.get_ref().clone(),
        server_api: server_api_tx.get_ref().clone(),
        api: client_api_tx.clone(),
        queue: Vec::new(),
    }));

    // tager sig af alle udefrakommende api kald aka. socket requests
    let client1 = client.clone();
    actix_web::rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let mut client = client1.write().unwrap();
                    // let mut client = get_client(client1);

                    // sender lige køen først
                    client.send_queue().await;
                    client.handle_ping_msg(&text).await;

                    let topic: Result<Topic, serde_json::Error> =
                        serde_json::from_str(&text.to_string());
                    if topic.is_err() || text == "ping" {
                        continue;
                    }
                    let topic = topic.unwrap();

                    // husker hvad texten er!
                    client.request = Some(text.to_string());

                    // håndtere requesten og formatere svaret
                    client.handle_topic(topic).await;
                }
                Ok(AggregatedMessage::Close(_reason)) => {
                    let client = client1.read().unwrap();
                    if let (Some(username), id) = (&client.username, client.id) {
                        client
                            .server_api
                            .send(ServerApi::ClientDestroy(Username::from(username), id))
                            .expect("kunne ikke sende");
                    }
                }
                _ => {}
            }
        }
    });

    // tager sig af alle interne api kald
    std::thread::spawn(move || {
        loop {
            match client_api_rx.try_recv() {
                Ok(msg) => match msg {
                    ClientApi::SendJSON(json) => {
                        let mut client = client.write().unwrap();
                        client.queue.push(json);
                    }
                },
                Err(_) => {}
            }

            // meget vigigt! ellers går cpu'en amok!
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
