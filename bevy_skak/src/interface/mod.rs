use bevy::{
    color::palettes::{
        css::{BEIGE, WHITE},
        tailwind::{AMBER_100, AMBER_200, GRAY_100, GRAY_200},
    },
    picking::pointer::PointerLocation,
    prelude::*,
};

use crate::{
    chess::{self, run_ifs::if_resources_exist, types::{ChessBoard, Index144, InvalidIndexes, MoveHistory, PlayMove, ValidMoves}},
    extra::{index_144_to_64, index_64_to_144, index_64_to_algebraic, index_to_pixel_coords},
};



mod types;
use types::*;



const WINDOW_SIZE: f32 = 512.;
const TILE_AMOUNT: usize = 8;



pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess Interface".into(),
                resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                position: WindowPosition::At((0, 300).into()),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Update,
            (
                (setup, spawn_pieces).run_if(if_resources_exist),
                (
                    remove_possible_moves,
                    spawn_possible_moves,
                    get_possible_moves,
                    // update_possible_moves,
                )
                    .chain(),
                update_tile_size_res,
                update_index,
                update_placement,

                hover_piece,
                select_piece,
                hightlight_hovered_piece,
                update_mouse_pos,
                selected_follows_mouse,
            ),
        )
        .insert_resource(WindowSize(WINDOW_SIZE))
        .insert_resource(TileAmount(TILE_AMOUNT))
        .insert_resource(MousePosition(Vec2::default()))
        .insert_resource(TileSize(WINDOW_SIZE as f32 / TILE_AMOUNT as f32));
    }
}


fn update_mouse_pos(mut mouse_pos: ResMut<MousePosition>, mouse: Query<&PointerLocation>, window_size: Res<WindowSize>) {
    if mouse.is_empty() { return }
    let mouse = mouse.single().unwrap();
    
    if let Some(location) = mouse.location() {
        let pos = location.position;

        // Sørg for at musens koordinater bliver oversat til verdens koordinater!
        let mpos = Vec2 {
            x: pos.x - window_size.0 / 2.,
            y: -(pos.y - window_size.0 / 2.),
        };

        mouse_pos.0 = mpos;
    }
}



fn select_piece(
    mut commands: Commands,
    mut chess_piece: Query<(Entity, &mut PieceIndex), (With<Hover>, With<ChessPiece>)>,
    highlighted_move: Query<(Entity, &PieceIndex), (With<Hover>, With<Highlight>, Without<ChessPiece>)>,
    mouse: Res<ButtonInput<MouseButton>>,

    mut play_move: Query<&mut PlayMove>,
    mut selected: Local<Option<Entity>>,
) {
    if mouse.just_released(MouseButton::Left) {
        println!(" -> Just released!");
        if let Some(entity) = *selected {
            if let Ok((_, mut index)) = chess_piece.get_mut(entity) {
                index.0 = index.0; // sætter brikken tilbage på plads
                commands.entity(entity).remove::<Selected>();
    
                if let Ok(hp) = highlighted_move.single() {
                    if let Ok(mut play_move) = play_move.single_mut() {
                        play_move.0 = Some(chess::types::Move {
                            from: index.0,
                            to: index.0,
                        })
                    }
                }
            }
        }
    }


    if get_len_of_iter(chess_piece.iter()) != 1 { return }
    let (entity, index) = chess_piece.single().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        println!(" -> Pressed!");
        commands.entity(entity).insert(Selected);
        *selected = Some(entity);
    }
}


fn selected_follows_mouse(
    mut pieces: Query<&mut Transform, With<Selected>>,
    mouse: Res<MousePosition>,

) {
    for mut transform in &mut pieces {
        transform.translation.x = mouse.x;
        transform.translation.y = mouse.y;
    }
}



fn hover_piece(
    pieces: Query<(Entity, &Transform, Option<&Hover>), With<IsHoverable>>,
    mut last_highlight: Local<Option<Entity>>,
    mouse: Res<MousePosition>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
) { 
    for (piece_entity, transform, optional_hover) in &pieces {
        let translation = transform.translation.xy();

        if mouse.x >= translation.x - tile_size.0 / 2.0 && mouse.x <= translation.x + tile_size.0 / 2.0 && mouse.y >= translation.y - tile_size.0 / 2.0 && mouse.y <= translation.y + tile_size.0 / 2.0 {
            if *last_highlight == Some(piece_entity) {
                continue;
            }

            *last_highlight = Some(piece_entity);
            
            // tilføj komponent
            commands.entity(piece_entity).try_insert(Hover);
        }
        else if optional_hover.is_some() {
            // hvis ikke musen er over brikken, og Hover er på, så skal den fjernes!
            commands.entity(piece_entity).remove::<Hover>();
        }
    }
}



fn hightlight_hovered_piece(
    mut commands: Commands,
    chess_pieces: Query<(Entity, Option<&Hover>, Option<&HasHoverIcon>), With<IsHoverable>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, is_hovering, has_icon) in &chess_pieces {
        match (is_hovering.is_some(), has_icon.is_some()) {
            (true, false) => {
                let child = commands.spawn((
                    Sprite {
                        image: asset_server.load("hover.png"),
                        ..default()
                    },
                    ChildOf(entity),
                    Transform::default(),
                )).id();

                commands.entity(entity).insert(HasHoverIcon(child));
            },
            (false, true) => {
                let child_entity = has_icon.unwrap().0;
                commands.entity(child_entity).despawn();
                commands.entity(entity).remove::<HasHoverIcon>();
            },
            (_, _) => {},
        }
    }
}


fn update_tile_size_res(
    mut tile_size: ResMut<TileSize>,
    window_size: Res<WindowSize>,
    tile_amount: Res<TileAmount>,
) {
    tile_size.0 = window_size.0 as f32 / tile_amount.0 as f32;
}

fn remove_possible_moves(
    mut commands: Commands,
    mut pmoves: Query<&mut PossibleMoves>,
    tiles: Query<Entity, With<Highlight>>,
    is_something_selected: Query<&Selected, With<ChessPiece>>,
) {
    if get_len_of_iter(pmoves.iter()) != 1 { return }
    if !is_something_selected.is_empty() { return }
    if tiles.is_empty() { return }
    let mut pmoves = pmoves.single_mut().unwrap();

    pmoves.0.clear();

    for entity in &tiles {
        commands.entity(entity).despawn();
    }
}

fn spawn_possible_moves(
    mut commands: Commands,
    query: Query<&PossibleMoves, Changed<PossibleMoves>>,
    asset_server: Res<AssetServer>,
    highlighted_tiles: Query<Entity, With<Highlight>>,

    tile_amount: Res<TileAmount>,
    window_size: Res<WindowSize>,
) {
    if let Ok(possible_moves) = query.single() {
        // sletter alle andre tiles
        for entity in &highlighted_tiles {
            commands.entity(entity).despawn();
        }
        
        for piece_index in &possible_moves.0 {
            commands.spawn((
                Name::new("Highlight"),
                Highlight,
                IsHoverable,
                piece_index.clone(),
                Sprite {
                    image: asset_server.load("highlight.png"),
                    ..default()
                },
                Transform::default()
                    .with_translation(index_to_pixel_coords(piece_index.0, tile_amount.0, window_size.0).into()),
            ));
        }
    }
}




fn get_possible_moves(
    mut pmoves: Query<&mut PossibleMoves>,
    piece: Query<&PieceIndex, (Added<Selected>, With<ChessPiece>)>,
    valid_moves: Query<&ValidMoves>,
    tile_amount: Res<TileAmount>,
) {
    if get_len_of_iter(piece.iter()) != 1 { return }
    if pmoves.is_empty() { return }
    if valid_moves.is_empty() { return }
    
    let mut pmoves = pmoves.single_mut().unwrap();
    let piece = piece.single().unwrap();
    let valid_moves = valid_moves.single().unwrap();

    let mut first_push = true;

    for vmove in &valid_moves.0 {
        if vmove.from.u12() == piece.0.u12() {
            // let i64 = vmove.to.i8();
            // if let Some(i64) = i64 {
            //     pmoves.0.push(PieceIndex(i64 as usize));
            // }

            // Hvis det her fejler betyder det at en move, som burde være blevet sorteret fra ikke gjorde...
            // Du prøver altså at rykke en brik ud af din 8x8 pladen. Dumt gjort - totalt utjekket
            
            if first_push && !pmoves.0.is_empty() {
                pmoves.0.clear();
            }
            first_push = false;

            pmoves.0.push(PieceIndex(vmove.to));
        }
    }
}


fn update_placement(
    mut pieces: Query<(&Name, &mut Transform, &PieceIndex), Changed<PieceIndex>>,

    tile_amount: Res<TileAmount>,
    window_size: Res<WindowSize>,
) {
    for (name, mut transform, index) in &mut pieces {
        transform.translation = index_to_pixel_coords(index.0, tile_amount.0, window_size.0).into();
    }
}

fn update_index(
    mut commands: Commands,
    mut pieces: Query<(Entity, &mut PieceIndex)>,
    history: Query<&mut MoveHistory, Changed<MoveHistory>>,
) {
    if let Ok(history) = history.single() {
        println!("changing...");
        for mv in &history.0 {
            for (entity, mut index) in &mut pieces {
                if index.0 == mv.from {
                    index.0 = mv.to;
                } else if index.0 == mv.to {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chessboard_query: Query<&ChessBoard, Added<ChessBoard>>,
    resolution: Res<WindowSize>,
    tile_amount: Res<TileAmount>,

    mut deb: Local<bool>,
) {
    if *deb { return }
    *deb = true;


    let mut index = Index144::new();


    if let Ok(chessboard) = chessboard_query.single() {
        for piece in &chessboard.0 {
            if let Some(piece) = piece {

                commands.spawn((
                    ChessPiece,
                    IsHoverable,
                    Name::new("Chess Piece"),
                    PieceIndex(index),
                    Sprite {
                        image: asset_server.load(format!("{}.png", piece.as_letters())),
                        ..default()
                    },
                    Transform::default()
                        .with_scale(Vec3::splat((resolution.0 / tile_amount.0 as f32) / 150.0))
                        .with_translation(
                            index_to_pixel_coords(
                                index,
                                tile_amount.0,
                                resolution.0,
                            )
                            .into(),
                        ),
                ));
            }

            index.inc(12);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resolution: Res<WindowSize>,
    tile_amount: Res<TileAmount>,

    mut deb: Local<bool>,
    invalid_indexes: Res<InvalidIndexes>,
) {
    if *deb {
        return;
    }
    *deb = true;

    commands.spawn(Camera2d);
    commands.spawn(PossibleMoves(Vec::new()));

    let tile_size = resolution.0 / tile_amount.0 as f32;
    let mut color = true;

    let mut index = Index144::from_minus_one();
    for x in 0..tile_amount.0 as i32 {
        for y in 0..tile_amount.0 as i32 {
            index.inc(tile_amount.0);
            
            
            if y != 0 {
                color = !color;
            }

            let mesh_color = match (invalid_indexes.contains(&index), color) {
                (true, true) => AMBER_100,
                (true, false) => GRAY_100,
                (false, true) => BEIGE,
                (false, false) => WHITE,
            };

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(mesh_color))),
                Transform::default()
                    .with_scale(Vec3::splat(resolution.0 / tile_amount.0 as f32))
                    .with_translation(
                        (
                            (x as f32 * tile_size) + tile_size / 2.0 - resolution.0 / 2.0,
                            (-y as f32 * tile_size) - tile_size / 2.0 + resolution.0 / 2.0,
                            0.0,
                        )
                            .into(),
                    ),
            ));
        }
    }
}




fn get_len_of_iter(something: impl Iterator) -> i32 {
    let mut len = 0;
    something.for_each(|_| len += 1);
    return len;
}