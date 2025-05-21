use bevy::prelude::*;
use crate::chess::chess_types::ChessBoard;



#[derive(Resource)]
pub struct BevyChessBoard(pub ChessBoard);




pub struct BevyChessPlugin;
impl Plugin for BevyChessPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BevyChessBoard(ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq")))
            .add_systems(PostUpdate, tick)
        ;

    }
}



fn tick(mut board: ResMut<BevyChessBoard>) {
    board.0.tick();
}