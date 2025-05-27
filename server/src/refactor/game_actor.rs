use chess_machine_lib::chess::chess_types::{ChessBoard, ChessColor, NamingConvention};
use super::*;


pub struct Game {
    id: usize,

    srv: Addr<Server>,
    white: String,
    black: String,

    chessboard: ChessBoard,

    time: TimeFormat,

    fen: String,
}

#[derive(Clone)]
pub struct TimeFormat {
    name: String,
    description: String,
    initial: Duration,
    increment: Duration,
}

impl TimeFormat {
    pub fn formats() -> Vec<TimeFormat> {
        vec![
            TimeFormat {
                name: "M5s3".to_string(),
                description: "5min game with a 3sec increment each turn".to_string(),
                initial: Duration::from_secs(5 * 60),
                increment: Duration::from_secs(5),
            },
            TimeFormat {
                name: "M10".to_string(),
                description: "10min game".to_string(),
                initial: Duration::from_secs(10 * 60),
                increment: Duration::from_secs(0),
            },
        ]
    }

    pub fn from(_timeformat: &str) -> TimeFormat {
        TimeFormat::default()
    }

    pub fn default() -> TimeFormat {
        // TimeFormat::formats()[0]
        TimeFormat {
            name: "M5s3".to_string(),
            description: "5min game with a 3sec increment each turn".to_string(),
            initial: Duration::from_secs(5 * 60),
            increment: Duration::from_secs(5),
        }
    }
}



impl Game {
    fn broadcast_state(&self) {
        self.srv.do_send(server::SendMessage::To(
            vec![self.white.clone(), self.black.clone()],
            OutgoingWsMsg::content("state", self.fen.clone()),
        ));
    }

    fn your_turn(&self) {
        let players_turn = match self.chessboard.turn {
            ChessColor::White => self.white.clone(),
            ChessColor::Black => self.black.clone(),
        };

        self.srv.do_send(ServerGameAPI::YourTurn(
            self.id,
            players_turn,
            self.fen.clone(),
            Duration::from_secs(1),
        ));
    }

    fn _make_move(&mut self, _move_name: &str) {
        // chess.load_fen(&self.fen)
        // let r = chess.play_move(move_name)
        /*

        if r {
            self.
        }


        */
        todo!()
    }

    pub fn new(id: &usize, srv: &Addr<Server>, p1: &str, p2: &str, format: &TimeFormat) -> Game {
        let mut chessboard = ChessBoard::default();
        chessboard.set_naming_convention(NamingConvention::LongAlgebraicNotation);
        
        Game {
            id: id.clone(),
            srv: srv.clone(),
            white: p2.to_string(),
            black: p1.to_string(),
            chessboard: chessboard,
            time: format.clone(),
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        }
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("GAME: started");

        self.broadcast_state();
        self.your_turn();
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.srv.do_send(server::Cleanup::Game(self.id))
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub enum API {
    GetState(String), // Username
    /// 1. move name
    Move(String),
}

impl Handler<API> for Game {
    type Result = bool;

    fn handle(&mut self, msg: API, _: &mut Self::Context) -> Self::Result {
        match msg {
            API::GetState(username) => {
                let fen = self.fen.clone();
                self.srv.do_send(server::SendMessage::To(
                    vec![username],
                    OutgoingWsMsg::content("state", fen),
                ));
            }
            API::Move(move_name) => {
                // hvis trækket er gyldigt, så...
                if self.chessboard.is_move_name_valid(&move_name) {
                    self.chessboard.play_notation(&move_name);
                    self.fen = self.chessboard.to_fen();
                    self.broadcast_state();
                    self.your_turn();
                }
            }
        }

        true
    }
}
