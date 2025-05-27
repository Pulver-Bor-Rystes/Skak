use bevy::prelude::*;
use chess_machine_lib::chess::chess_types::ChessBoard;



#[derive(Resource)]
pub struct BevyChessBoard(pub ChessBoard);




pub struct BevyChessPlugin;
impl Plugin for BevyChessPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BevyChessBoard(ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0")))
        ;
    }
}