use bevy::prelude::*;

use crate::chess::types::Index144;




#[derive(Resource)]
pub struct WindowSize(pub f32);


#[derive(Resource)]
pub struct TileAmount(pub usize);


#[derive(Resource, DerefMut, Deref)]
pub struct TileSize(pub f32);


#[derive(Component, Clone)]
pub struct PieceIndex(pub Index144);


#[derive(Component)]
pub struct ChessPiece;


#[derive(Component)]
pub struct PossibleMoves(pub Vec<PieceIndex>);


#[derive(Component)]
pub struct Highlight;


#[derive(Component)]
pub struct Selected;



#[derive(Component)]
pub struct Hover;


#[derive(Component)]
pub struct HasHoverIcon(pub Entity);



#[derive(Resource, Deref)]
pub struct MousePosition(pub Vec2);


#[derive(Component)]
pub struct IsHoverable;