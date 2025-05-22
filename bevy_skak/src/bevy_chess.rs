use bevy::prelude::*;
use crate::chess::chess_types::ChessBoard;



#[derive(Resource)]
pub struct BevyChessBoard(pub ChessBoard);




pub struct BevyChessPlugin;
impl Plugin for BevyChessPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BevyChessBoard(ChessBoard::from_fen("r3k2r/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")))
        ;
    }
}