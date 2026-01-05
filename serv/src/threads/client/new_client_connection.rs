use std::{sync::{mpsc::{self, Sender}, Arc, RwLock}, time::Duration};
use actix_web::{rt, web::{self, Data}, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use sqlx::{MySql, Pool};
use crate::{error, from_json, info, threads::{client::{ClientApi, ClientConn, Output}, server::{ServerApi, Username}}, to_json, warn, Topic, TopicContent};



pub async fn new_socket_conn(req: HttpRequest, stream: web::Payload, db: Data<Pool<MySql>>, server_api_tx: Data<Sender<ServerApi>>) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    // api input output
    let (client_api_tx, client_api_rx) = mpsc::channel::<ClientApi>();

    // selve client conn objektet
    let client =
        Arc::new(
            RwLock::new(
                ClientConn {
                    id: None,
                    username: None,

                    // meta stuff
                    request: None,
                    session,
                    db: db.get_ref().clone(),
                    server_api: server_api_tx.get_ref().clone(),
                    api: client_api_tx.clone(),
                    queue: Vec::new(),
                }
            )
        );

    

    // tager sig af alle udefrakommende api kald aka. socket requests
    let client1 = client.clone();
    rt::spawn(async move {
        match server_api_tx.send(ServerApi::ClientNew(client_api_tx.clone())) {
            Ok(_) => {},
            Err(e) => error!("{e}"),
        }

        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let mut client = client1.write().unwrap();

                    // sender lige køen først
                    for msg in client.queue.clone() {
                        client.session.text(msg).await.expect("Kunne ikke sende en besked fra køen");
                    }
                    client.queue.clear();

                    
                    if text == "ping" {
                        let pong_msg = to_json!(Topic { topic: "pong".into() });
                        client.session.text(pong_msg).await.unwrap();    
                        continue;
                    }

                    let topic: Result<Topic, serde_json::Error> = serde_json::from_str(&text.to_string());
                    if topic.is_err() { continue; }
                    let topic = topic.unwrap();

                    // husker hvad texten er!
                    client.request = Some(text.to_string());
                    
                    // håndtere requesten og formatere svaret
                    client.handle_topic(topic).await;
                }
                Ok(AggregatedMessage::Close(_reason)) => {
                    let client = client1.read().unwrap();
                    if let (Some(username), Some(id)) = (&client.username, client.id){
                        client.server_api.send(ServerApi::ClientDestroy(Username::from(username), id)).expect("kunne ikke sende");
                    }
                },
                _ => {}
            }
        }
    });

    // tager sig af alle interne api kald
    std::thread::spawn(move || {
        loop {
            match client_api_rx.try_recv() {
                Ok(msg) => match msg {
                    ClientApi::Id(id) => {
                        let mut client = client.write().unwrap();
                        client.id = Some(id);
                    },
                    ClientApi::SendJSON(json) => {
                        let mut client = client.write().unwrap();
                        client.queue.push(json);
                    },
                },
                Err(_) => {},
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}



impl ClientConn {
    async fn handle_topic(&mut self, topic: Topic) {
        
        // forsøger at håndtere requesten!
        let handle_response = self.distribute(topic.clone()).await;

        match handle_response {
            Ok(output) => {
                if output == Output::Pending { return; }
            
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
