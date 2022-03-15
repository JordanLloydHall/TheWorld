// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use the_world::GamePlugin;

#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
        app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb_u8(141, 196, 53)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "The World".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);
        
    
        #[cfg(feature = "dev")]
        // Debug hierarchy inspector
        app.add_plugin(WorldInspectorPlugin::new());
    
        app.run();
}
