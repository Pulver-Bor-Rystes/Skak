use bevy::{color::palettes::css::{BEIGE, WHITE}, picking::pointer::PointerLocation, prelude::*};
use crate::{bevy_chess::BevyChessBoard, chess::chess_types::{BoardType, Index144}, extra::index_to_pixel_coords};
use super::types::*;
use select::*;

pub mod select;
pub mod hover;
pub mod placement;
pub mod possible_moves;


#[derive(Event)]
pub struct FlipUIEvent;

#[derive(Event)]
pub struct ReRenderBoard;

#[derive(Resource)]
pub struct UIOrientation(pub bool);


pub fn setup_camera(mut commands: Commands, mut ev: EventWriter<ReRenderBoard>) {
    commands.spawn(Camera2d);
    ev.write(ReRenderBoard);
}


pub fn turn_ui_around(mut ev: EventWriter<FlipUIEvent>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyX) {
        ev.write(FlipUIEvent);
    }
}


pub fn on_turn_ui_around(
    mut uio: ResMut<UIOrientation>,
    mut ev: EventReader<FlipUIEvent>,
    mut rerender_ev: EventWriter<ReRenderBoard>,
    mut tiles: Query<&mut Index, With<Tile>>
) {
    for _ in ev.read() {
        for mut tile in &mut tiles {
            let v = tile.0.i12();
            tile.0.set_12(143 - v);
        }

        uio.0 = !uio.0;
        rerender_ev.write(ReRenderBoard);
    }
}


pub fn setup_black_white_tiles(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_size: Res<WindowSize>,
) {
    let tile_size = window_size.0 / 8.0;
    let mut color = false;

    let mut index = Index144::from_minus_one();
    // let mut index = Index144::from12(144);
    for y in 0..8 as i32 {
        for x in 0..8 as i32 {
            index.inc(BoardType::Regular);
            // index.dec(BoardType::Regular);
            
            if x != 0 {
                color = !color;
            }

            let mesh_color = match color {
                true => BEIGE,
                false => WHITE,
            };

            let parent = commands.spawn((
                Name::new(format!("Tile: ({})", index.to_str())),
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



pub fn on_board_change(
    chessboard: Res<BevyChessBoard>,
    mut ev: EventWriter<ReRenderBoard>,
) {
    if !chessboard.0.board_changed { return }
    ev.write(ReRenderBoard);
}



pub fn remove_chess_pieces(
    mut commands: Commands,
    ev: EventReader<ReRenderBoard>,
    pieces: Query<Entity, With<ChessPiece>>,
) {
    if ev.is_empty() { return }

    for entity in &pieces {
        commands.entity(entity).despawn();
    }
}


pub fn spawn_chess_pieces(
    mut commands: Commands,
    board: Res<BevyChessBoard>,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
    ui_orientation: Res<UIOrientation>,
    ev: EventReader<ReRenderBoard>
) {
    if ev.is_empty() { return }
    
    let mut index = Index144::new();
    for piece in &board.0.pieces {
        if let Some(piece) = piece {
            let parent = commands.spawn((
                Name::new(format!("Chess Piece - {}, has_moved: {}", piece.to_str_img_format(), piece.has_moved)),
                ChessPiece,
                IsHoverable,
                Visibility::default(),
                Index(index),
                Transform::default()
                    .with_translation(
                        index_to_pixel_coords(index, window_size.0, ui_orientation.0).into(),
                    ),
            )).id();

            // actual image
            commands.spawn((
                Name::new(format!("Piece Image - {}", piece.to_str_img_format())),
                ChildOf(parent),
                Visibility::default(),
                Sprite {
                    image: asset_server.load(format!("{}.png", piece.to_str_img_format())),
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


// TODO: Lav noget der ændre på størrelsen af vinduet
// pub fn update_window_size(
//     mut ws: ResMut<WindowSize>,
// ) {

// }