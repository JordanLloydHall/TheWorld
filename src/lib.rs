mod actions;
mod audio;
// mod burrow;
mod loading;
mod menu;
mod player;
// mod rabbit;
mod house;
mod human;
mod physics;
mod territory;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
// use burrow::BurrowPlugin;
use house::HousePlugin;
use human::HumanPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
// use territory::TerritoryPlugin;

use physics::AddedObject;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use heron::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            // .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            // .add_plugin(BurrowPlugin)
            // .add_plugin(TerritoryPlugin)
            .add_plugin(HousePlugin)
            .add_plugin(HumanPlugin)
            .add_plugin(PhysicsPlugin::default())
            .insert_resource(AddedObject(false));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
