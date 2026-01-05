use crate::{parsing::Json, types::Username};
use chess_machine_lib::chess::chess_types::{ChessBoard, NamingConvention};
use rand::seq::IndexedRandom;

/// Et spil bliver gemt på serveren thread, hvor den hurtigt kan tilgås.
/// Selve game skal sådan set bare holde på nogle rimelig statiske oplysninger.
/// Der sker altså ikke noget særligt spændende her. Spillet skal spilles af to spillere,
/// så sekundet et træk er blevet lavet og godkendt, skal den nye spiller have af vide hvad der skete, så de kan foretage et træk
#[derive(Clone, Debug)]
pub struct Game {
    pub id: u64,
    pub players: [Username; 2],
    white: Username,
    board: ChessBoard,
}

// pub struct Details {
//     game_hash: String,
//     white: String,
//     black: String,
//     current_fen: String,
//     move_history: Vec<String>,
// }

impl Json for &Game {
    fn to_json(&self) -> String {
        (*self).to_json()
    }
}

impl Json for Game {
    fn to_json(&self) -> String {
        let black = if self.white == self.players[0] {
            &self.players[1]
        } else {
            &self.players[0]
        };

        let move_history: Vec<String> = self
            .board
            .move_history
            .iter()
            .map(|m| m.name.clone())
            .collect();

        format!(
            "{{ \"hash\": {}, \"white\": {}, \"black\": {}, \"current_fen\": {}, \"move_history\": {} }}",
            self.id.to_json(),
            self.white.to_json(),
            black.to_json(),
            self.board.to_fen().to_json(),
            move_history.to_json(),
        )
    }
}

impl Game {
    pub fn new(id: u64, players: [Username; 2]) -> Self {
        let game = Game {
            id,
            players: players.clone(),
            white: players.choose(&mut rand::rng()).unwrap().clone(),
            board: ChessBoard::default()
                .set_naming_convention(NamingConvention::LongAlgebraicNotation)
                .to_owned(),
        };

        // fortæl spillere at de er med i et spil

        game
    }

    pub fn players(&self) -> Vec<&Username> {
        return vec![&self.players[0], &self.players[1]];
    }

    pub fn does_player_need_to_know(&self, username: &Username) -> bool {
        self.players.contains(username)
    }

    pub fn play_move(&mut self, move_name: &str) -> Result<(), String> {
        match self.board.is_move_name_valid(move_name) {
            true => Ok(self.board.play_notation(move_name)),
            false => Err(format!("Invalid move: {move_name}")),
        }
    }
}
