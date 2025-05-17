use bevy::{color::palettes::{css::{BEIGE, WHITE}, tailwind::{AMBER_100, GRAY_100}}, picking::pointer::PointerLocation, platform::collections::HashSet, prelude::*};
use crate::{chess::chess_types::{BoardType, ChessBoard, Index144, InvalidIndexes}, extra::index_to_pixel_coords};
use super::types::*;
use select::*;

pub mod select;
pub mod hover;
pub mod placement;
pub mod possible_moves;
pub mod react_on_board_changes;


pub fn setup_black_white_tiles(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_size: Res<WindowSize>,
) {
    commands.spawn(Camera2d);

    let tile_size = window_size.0 / 8.0;
    let mut color = false;

    let mut index = Index144::from_minus_one();
    for y in 0..8 as i32 {
        for x in 0..8 as i32 {
            index.inc(BoardType::Regular);
            
            if x != 0 {
                color = !color;
            }

            let mesh_color = match color {
                true => BEIGE,
                false => WHITE,
            };

            let parent = commands.spawn((
                Name::new(format!("Tile: ({})", index.str())),
                Tile,
                Index(index),
                IsHoverable,
                Visibility::default(),
                Transform::default()
                    .with_translation(
                        (
                            (x as f32 * tile_size) + tile_size / 2.0 - window_size.0 / 2.0,
                            (-y as f32 * tile_size) - tile_size / 2.0 + window_size.0 / 2.0,
                            0.0,
                        )
                            .into(),
                    )
                    
            )).id();


            commands.spawn((
                Name::new("Tile Mesh"),
                ChildOf(parent),
                Visibility::default(),
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(mesh_color))),
                Transform::default()
                    .with_scale(Vec3::splat(window_size.0 / 8.0))
            ));
        }
    }
}


pub fn remove_chess_pieces(
    mut commands: Commands,
    board_change: Query<(), Changed<ChessBoard>>,
    pieces: Query<Entity, With<ChessPiece>>,
) {
    if board_change.is_empty() { return }

    for entity in &pieces {
        commands.entity(entity).despawn();
    }
}


pub fn spawn_chess_pieces(
    mut commands: Commands,
    board: Query<&ChessBoard, Changed<ChessBoard>>,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    if board.is_empty() { return }
    let board = board.single().unwrap();
    
    let mut index = Index144::new();
    for piece in &board.pieces {
        if let Some(piece) = piece {
            let parent = commands.spawn((
                Name::new(format!("Chess Piece - {}, has_moved: {}", piece.as_letters(), piece.has_moved)),
                ChessPiece,
                IsHoverable,
                Visibility::default(),
                Index(index),
                Transform::default()
                    .with_translation(
                        index_to_pixel_coords(index, window_size.0).into(),
                    ),
            )).id();

            // actual image
            commands.spawn((
                Name::new(format!("Piece Image - {}", piece.as_letters())),
                ChildOf(parent),
                Visibility::default(),
                Sprite {
                    image: asset_server.load(format!("{}.png", piece.as_letters())),
                    ..default()
                },
                Transform::default()
                    .with_scale(Vec3::splat((window_size.0 / 8.0) / 150.0)),
            ));
        }

        index.inc(BoardType::Large);
    }
}





pub fn update_mouse_pos(mut mouse_pos: ResMut<MousePosition>, mouse: Query<&PointerLocation>, window_size: Res<WindowSize>) {
    if mouse.is_empty() { return }
    let mouse = mouse.single().unwrap();
    
    if let Some(location) = mouse.location() {
        let pos = location.position;

        // Sørg for at musens koordinater bliver oversat til verdens koordinater!
        let mpos = Vec2 {
            x: pos.x - window_size.0 / 2.,
            y: -(pos.y - window_size.0 / 2.),
        };

        mouse_pos.0 = Some(mpos);
    }
}


pub fn update_tile_size(mut tile_size: ResMut<TileSize>, window_size: Res<WindowSize>) {
    tile_size.0 = window_size.0 / 8.0;
}