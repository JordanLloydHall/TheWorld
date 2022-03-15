use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct PlayerPlugin;

// enum Object {

// }

// const OBJECTS: [] = 

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
                    .with_system(scale_player_camera)
                    .with_system(place_object)
                    // .with_system(update_selected_object),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

// fn update_selected_object(
//     item: 
//     mut scroll_evr: EventReader<MouseWheel>,
// ) {
//     use bevy::input::mouse::MouseScrollUnit;
//     for ev in scroll_evr.iter() {
//         match ev.unit {
//             MouseScrollUnit::Line => {
//                 println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
//             }
//             MouseScrollUnit::Pixel => {
//                 println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
//             }
//         }
//     }
// }

fn place_object(
    mut commands: Commands,
    // need to get window dimensions
    wnds: Res<Windows>,
    // Sprites
    sprites: Res<TextureAssets>,
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to
    let wnd = wnds.get(camera.window).unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos = world_pos.truncate();

        if buttons.just_pressed(MouseButton::Left) {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.sprites.clone(),
                transform: Transform::from_translation(world_pos.extend(0.)),
                sprite: TextureAtlasSprite::new(526),
                ..Default::default()
            });
        }
        
    }

}


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
