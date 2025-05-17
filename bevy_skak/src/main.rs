use bevy::prelude::*;
use chess::ChessPlugin;
use interfacev2::InterfacePlugin;


mod chess;
// mod interface;
mod interfacev2;
mod extra;


fn main() {
    App::new()
        .add_plugins((
            InterfacePlugin,
            ChessPlugin,
        ))
        .run();
}