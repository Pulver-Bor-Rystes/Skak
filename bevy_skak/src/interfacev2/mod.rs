use bevy::prelude::*;
use types::{MousePosition, TileSize, WindowSize};
use ui::{hover::spawn_hightlight_on_hovered, placement::update_placement, possible_moves::{remove_hightlights, spawn_highlights}, select::*, *};


mod ui;
mod types;

const WINDOW_SIZE: f32 = 612.;


pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chess Interface".into(),
                    resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                    position: WindowPosition::At((0, 300).into()),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
            .add_event::<DeselectEvent>()
            .add_event::<FlipUIEvent>()
            .add_event::<ReRenderBoard>()
            .insert_resource(WindowSize(WINDOW_SIZE))
            .insert_resource(TileSize(64.0))
            .insert_resource(MousePosition(None))
            .insert_resource(UIOrientation(true))
            .add_systems(Startup, (setup_black_white_tiles, setup_camera))
            .add_systems(PreUpdate, (update_mouse_pos, remove_chess_pieces))
            .add_systems(Update, (
                // ui
                update_tile_size,
                ui::hover::hover,
                spawn_hightlight_on_hovered,
                (spawn_chess_pieces).chain(),
                turn_ui_around,
                on_turn_ui_around,
                on_board_change,

                // select
                select_piece,
                move_selected_piece,
                panic_if_multiple_pieces_are_selected,
                deselect_piece,

                // placement
                update_placement,

                // possible_moves.rs
                (remove_hightlights, spawn_highlights).chain(),
                

            ))
        ;
    }
}

