use crate::{bevy_chess::BevyChessBoard, extra::iter_len};

use super::*;

#[derive(Event)]
pub struct DeselectEvent;


pub fn select_piece(
    mut commands: Commands,
    mut hovered_pieces: Query<Entity, (With<Hover>, With<ChessPiece>, Without<Selected>)>,
    selected_piece: Query<(), (With<Selected>, With<ChessPiece>)>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
) {
    if iter_len(hovered_pieces.iter()) != 1 { return }
    if !selected_piece.is_empty() { return }
    let entity = hovered_pieces.single_mut().unwrap();

    if mouse_btn.just_pressed(MouseButton::Left) {
        commands.entity(entity).insert(Selected);
    }
}



pub fn move_selected_piece(
    mouse: Res<MousePosition>,
    mut selected_piece: Query<&mut Transform, (With<Selected>, With<ChessPiece>)>,
) {
    if iter_len(selected_piece.iter()) != 1 { return }
    let mut t = selected_piece.single_mut().unwrap();

    if let Some(mouse) = mouse.0 {
        t.translation.x = mouse.x;
        t.translation.y = mouse.y;
    }
}


pub fn panic_if_multiple_pieces_are_selected(q: Query<&Name, (With<Selected>, With<ChessPiece>)>) {
    if iter_len(q.iter()) > 1 {
        for n in &q {
            warn!("Entity ({}) should not be selected!", n);
        }

        panic!("For mange selected brikker!")
    }
}


pub fn deselect_piece(
    mut commands: Commands,
    mut selected_piece: Query<(Entity, &mut Index), (With<Selected>, With<ChessPiece>)>,
    hovered_tiles: Query<&Index, (With<Hover>, With<Tile>, Without<ChessPiece>)>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut ev_writer: EventWriter<DeselectEvent>,

    // mut chessboard_query: Query<(&ValidMoves, &mut MoveHistory)>,
    mut chessboard: ResMut<BevyChessBoard>,
) {
    if iter_len(selected_piece.iter()) != 1 { return }
    let (entity, mut from_index) = selected_piece.single_mut().unwrap();
    
    if mouse_btn.just_released(MouseButton::Left) {
        commands.entity(entity).remove::<Selected>();

        if iter_len(hovered_tiles.iter()) == 1 {
            if let Ok(to_index) = hovered_tiles.single() {
                info!("Trying to place at: {}", to_index.0.str());

                // let (valid_moves, mut history) = chessboard_query.single_mut().unwrap();
                for vm in &chessboard.0.valid_moves.clone() {
                    if vm.to() == to_index.0 && vm.from() == from_index.0 {
                        info!("Playing move!: {:?}", vm);
                        // chessboard.0.move_history.push(vm.clone());
                        chessboard.0.play_move(vm);
                    }
                }
                
            }
        }
        
        ev_writer.write(DeselectEvent);
        from_index.0 = from_index.0;
    }
}
