use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

// pub struct RabbitPlugin;

#[derive(Component)]
pub struct Rabbit;

// This plugin handles player related stuff like movement
// Player logic is only active during the State `GameState::Playing`
// impl Plugin for RabbitPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(
//             SystemSet::on_update(GameState::Playing)
//                 .with_system(spawn_rabbit),
//         );
//     }
// }
