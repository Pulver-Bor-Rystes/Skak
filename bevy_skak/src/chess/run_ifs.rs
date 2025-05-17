use bevy::prelude::*;
use super::chess_types::InvalidIndexes;


pub fn if_resources_exist(res: Option<Res<InvalidIndexes>>) -> bool {
    res.is_some()
}