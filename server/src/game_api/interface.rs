use crate::communication::{server::Server, session::SessionContext};
use actix::Addr;
use rand::Rng;
use serde_json::Error as JsonError;



pub fn handle(ctx: &mut SessionContext) -> Option<()> {
    let res: Result<(), JsonError> = match ctx.topic.as_str() {
        _ => return None
    };

    match res {
        Ok(_) => Some(()),
        Err(_) => None,
    }
} 


fn handle_new_game(ctx: &mut SessionContext) -> Result<(), JsonError> {
    
}


pub struct Game {
    white_id: usize,
    black_id: usize,

    server: Addr<Server>,
    fen: String,
}

impl Game {
    pub fn new(players: [usize; 2], server: Addr<Server>) -> Game {
        let mut rng = rand::thread_rng();

        let white = players[0];
        let black = players[1];

        let r: usize = rng.gen_range(0..=1);

        let white = if r == 1 {
            black
        }
        else {
            white
        }

        let black = if r == 1 {
            white
        }
        else {
            black
        }


        // nu da farverne blevet valgt!
        // så skal vi have sat resten op :)

        Game {
            white_id: white,
            black_id: black,
            server,
            fen: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        }
    }
}
