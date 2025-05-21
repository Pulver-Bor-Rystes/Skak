use crate::{bevy_chess::BevyChessBoard, extra::iter_len};
use super::*;

#[derive(Component)]
pub struct ValidMove;




pub fn remove_hightlights(
    mut commands: Commands,
    mut deselect_reader: EventReader<DeselectEvent>,
    highlights: Query<Entity, With<ValidMove>>,
) {
    for _ev in deselect_reader.read() {
        for entity in &highlights {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_highlights(
    mut commands: Commands,
    bevy_chessboard: Res<BevyChessBoard>,
    selected_piece: Query<&Index, (With<ChessPiece>, Added<Selected>)>,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
    ui_orientation: Res<UIOrientation>,
) {
    if iter_len(selected_piece.iter()) != 1 {
        return;
    }

    let piece = selected_piece.single().unwrap();

    for valid_move in &bevy_chessboard.0.moves {
        if valid_move.from() != piece.0 {
            continue;
        }

        commands.spawn((
            Name::new("Valid Move Hightlight"),
            ValidMove,
            Transform::default()
                .with_translation(index_to_pixel_coords(valid_move.to(), window_size.0, ui_orientation.0).into(),)
                .with_scale(Vec3::splat((window_size.0 / 16.0) / 64.0)),
            Visibility::default(),
            Sprite {
                image: asset_server.load("highlight.png"),
                ..default()
            },
        ));
    }
}
