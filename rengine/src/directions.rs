use crate::{piece::PieceType, MoveRules};

pub fn get_directions_for_piece(kind: PieceType) -> Vec<((i32, i32), MoveRules)> {
    let r = MoveRules::default();

    let with_limit = MoveRules {
        limit: true,
        ..Default::default()
    };

    let only_capture = MoveRules {
        limit: true,
        only_capture: true,
        only_empty: false,
    };

    let only_empty = MoveRules {
        limit: true,
        only_capture: false,
        only_empty: true,
    };

    match kind {
        PieceType::Pawn => vec![((0, 1), only_empty), ((0, 2), only_empty), ((1, 1), only_capture), ((-1, 1), only_capture)],
        PieceType::Knight => vec![((2, 1), with_limit), ((2, -1), with_limit), ((-2, 1), with_limit), ((-2, -1), with_limit), ((1, 2), with_limit), ((1, -2), with_limit), ((-1, 2), with_limit), ((-1, -2), with_limit)],
        PieceType::Bishop => vec![((1, 1), r), ((-1, -1), r), ((-1, 1), r), ((1, -1), r)],
        PieceType::Rook => vec![((1, 0), r), ((0, 1), r), ((-1, 0), r), ((0, -1), r)],
        PieceType::Queen => vec![((1, 0), r), ((-1, 0), r), ((0, 1), r), ((0, -1), r), ((1, 1), r), ((-1, -1), r), ((-1, 1), r), ((1, -1), r)],
        PieceType::King => vec![((1, 0), with_limit), ((-1, 0), with_limit), ((0, 1), with_limit), ((0, -1), with_limit), ((1, 1), with_limit), ((-1, -1), with_limit), ((-1, 1), with_limit), ((1, -1), with_limit)],
    }
}