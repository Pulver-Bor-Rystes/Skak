use crate::{chess::chess_types::{Move, PlayMove}, extra::iter_len};

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

    mut play_move: Query<&mut PlayMove>,
) {
    if iter_len(selected_piece.iter()) != 1 { return }
    let (entity, mut index) = selected_piece.single_mut().unwrap();
    
    if mouse_btn.just_released(MouseButton::Left) {
        commands.entity(entity).remove::<Selected>();

        if iter_len(hovered_tiles.iter()) == 1 {
            if let Ok(tile) = hovered_tiles.single() {
                info!("Trying to place at: {}", tile.0.str());

                if let Ok(mut play_move) = play_move.single_mut() {
                    play_move.0 = Some(Move {
                        from: index.0,
                        to: tile.0,
                    });
                }
            }
        }
        
        ev_writer.write(DeselectEvent);
        index.0 = index.0;
    }
}