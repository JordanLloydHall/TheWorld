use super::actions::Actions;
use super::loading::TextureAssets;
use super::GameState;
use bevy::prelude::*;

// pub struct TerritoryPlugin;

#[derive(Component)]
pub struct Territory;

// This plugin handles player related stuff like movement
// Player logic is only active during the State `GameState::Playing`
// impl Plugin for TerritoryPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource()
//     }
// }
