use crate::{BitMove, HistoryHeuristic, PIECE_TYPE_COUNT, KillerMoves, Position, Score, TTNodeType, TranspositionTable};

#[allow(unused_imports)]
use crate::{Color, MoveMasks, Piece, MoveFlag};

// Most valuable victim - least valuable attacker [attacker][victim]
const MVV_LVA: [[i16; PIECE_TYPE_COUNT]; PIECE_TYPE_COUNT] = [
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
];

pub struct EvalMove;

impl EvalMove {
    #[inline(always)]
    pub fn eval(position: &Position, bit_move: BitMove) -> Score {
        let mut score = Score::ZERO;
        let source = bit_move.source();
        let target = bit_move.target();
        let piece = position.get_piece(source);
        let capture_option = position.get_piece_option(target);

        #[cfg(feature = "move_flag_eval")]
        {
            score += match bit_move.flag_option() {
                None | Some(MoveFlag::WDoublePawn) | Some(MoveFlag::BDoublePawn) => 0,
                Some(MoveFlag::WKCastle) | Some(MoveFlag::BKCastle) => 200, 
                Some(MoveFlag::WQCastle) | Some(MoveFlag::BQCastle) => 50,
                Some(MoveFlag::WEnPassant) | Some(MoveFlag::BEnPassant) => 150,
                Some(MoveFlag::PromoQ) => 500,
                Some(MoveFlag::PromoR) | Some(MoveFlag::PromoB) | Some(MoveFlag::PromoN) => -100,
            };
        }

        // NOTE: Although the following idea seems logical, it yields 10-20% worse performance!
        // score += EvalPosition::get_base_piece_position_score(piece, target, position.side) - EvalPosition::get_base_piece_position_score(piece, source, position.side); 

        if let Some(capture) = capture_option {
            score += MVV_LVA[piece][capture];

            #[cfg(feature = "capture_with_check_eval")]
            {
                let enemy_king_bb = match position.side {
                    Color::White => position.bitboards[Piece::BK],
                    Color::Black => position.bitboards[Piece::WK],
                };
                if (MoveMasks::get_piece_mask(piece, target, position.all_occupancy) & enemy_king_bb).is_not_empty() {
                    score += 300
                }
            }
        };

        #[cfg(feature = "eval_tt")]
        {
            if let Some(tt_data) = TranspositionTable::probe(position.zobrist_key) {
                if tt_data.best_move.bit_move == bit_move {
                    match tt_data.node_type {
                        TTNodeType::Exact => score += 10000,
                        TTNodeType::LowerBound => score += 4000,
                        TTNodeType::UpperBound => score += 3000,
                    }
                }
            }
        }

        #[cfg(feature = "killer_heuristic")]
        {
            if KillerMoves::get_primary(position.ply) == Some(bit_move) {
                score += 2000;
            } else if KillerMoves::get_secondary(position.ply) == Some(bit_move) {
                score += 1000;
            }
        }

        #[cfg(feature = "history_heuristic")]
        {
            score += HistoryHeuristic::get(position.side, source, target);
        }

        score
    }
}
