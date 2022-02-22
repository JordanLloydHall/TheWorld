use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerCamera;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player_camera)
                    .with_system(scale_player_camera),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

// fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
//     commands
//         .spawn_bundle(SpriteBundle {
//             texture: textures.rabbit.clone(),
//             transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
//             ..Default::default()
//         })
//         .insert(Player);
// }

fn move_player_camera(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Some(mut movement) = actions.player_movement {
        let move_speed = 150.;
        movement *= move_speed * time.delta_seconds();

        for mut player_transform in player_query.iter_mut() {
            player_transform.translation += movement;
        }
    }
}

fn scale_player_camera(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut OrthographicProjection, With<PlayerCamera>>,
) {
    if let Some(mut zoom) = actions.player_zoom {
        let zoom_speed = 1.;
        zoom *= zoom_speed * time.delta_seconds();

        for mut player_proj in player_query.iter_mut() {
            player_proj.scale += zoom;
            player_proj.scale = player_proj.scale.max(0.1);
        }
    }
}
