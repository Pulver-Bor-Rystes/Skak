use bevy::prelude::*;
use crate::chess::chess_types::ChessBoard;



#[derive(Resource)]
pub struct BevyChessBoard(pub ChessBoard);




pub struct BevyChessPlugin;
impl Plugin for BevyChessPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BevyChessBoard(ChessBoard::default()))
            .add_systems(PostUpdate, tick)
        ;

    }
}



fn tick(mut board: ResMut<BevyChessBoard>) {
    board.0.tick();
}