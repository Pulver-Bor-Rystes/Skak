use bevy::prelude::*;
use chess_machine_lib::chess::chess_types::Index144;



#[derive(Resource)]
pub struct WindowSize(pub f32);

#[derive(Resource, Deref)]
pub struct MousePosition(pub Option<Vec2>);

#[derive(Resource, DerefMut, Deref)]
pub struct TileSize(pub f32);




#[derive(Component)]
pub struct ChessPiece;


#[derive(Component)]
pub struct Hover;

#[derive(Component)]
pub struct Tile;


#[derive(Component)]
pub struct IsHoverable;


#[derive(Component)]
pub struct HasHoverIcon(pub Entity);


#[derive(Component)]
pub struct Index(pub Index144);



#[derive(Component)]
pub struct Selected;