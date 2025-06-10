use std::io::Error;
use shakmaty::{fen::Fen, CastlingMode, Chess};
use shakmaty_syzygy::Tablebase;

use crate::{BitMove, ScoringMove, FenString, Legal, MoveGeneration, Position, Score, Uci};

pub(crate) struct SyzygyTablebase {
    shakmaty_tablebase: Tablebase<Chess>
}

impl SyzygyTablebase {
    pub(crate) fn from_directory(path: &str) -> Result<SyzygyTablebase, Error> {
        let mut shakmaty_tablebase = Tablebase::new();
        shakmaty_tablebase.add_directory(path)?;
        Ok(SyzygyTablebase { shakmaty_tablebase })
    }

    pub(crate) fn best_move(&self, position: &Position) -> Option<ScoringMove> {
        let shakmaty_position = FenString::from(position).to_string()
            .parse::<Fen>().ok()?
            .into_position::<Chess>(CastlingMode::Standard).ok()?;

        let best_move = self.shakmaty_tablebase.best_move(&shakmaty_position).ok()?;
        
        match best_move {
            Some((chess_move, maybe_rounded_dtz)) => {
                let move_string = chess_move.to_uci(CastlingMode::Standard).to_string();
                
                // NOTE: The conditions under which the score should be negated are unclear
                let mut score = if maybe_rounded_dtz.is_zero() { Score::DRAW } else if maybe_rounded_dtz.is_positive() { -Score::CHECKMATE } else { Score::CHECKMATE };
                score += maybe_rounded_dtz.ignore_rounding().0 as i16;
                Some(ScoringMove::new(Uci::parse_move_string(&MoveGeneration::generate_moves::<BitMove, Legal>(position), &move_string).ok()?, score))
            },
            None => None,
        }
    }

    #[inline(always)]
    pub(crate) fn get_max_pieces(&self) -> usize {
        self.shakmaty_tablebase.max_pieces()
    }
}
