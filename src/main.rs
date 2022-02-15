// extern crate rapier2d as rapier; // For the debug UI.

use bevy::prelude::*;
use heron::prelude::*;

// use ui::DebugUiPlugin;

// #[path = "../../src_debug_ui/mod.rs"]
// mod ui;

const SCALE: f32 = 10.0;

#[derive(Component)]
struct Player(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0)))
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_graphics(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));
    commands.spawn_bundle(camera);
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */

    let ground_size = 5000.0;
    let ground_height = 1.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(ground_size, ground_height)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(ground_size, 0f32),
            heights: vec![vec![-ground_height, ground_height]],
        });

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 40.0;

    let shift = rad + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    let mut color = 0;

    for j in 0usize..100 {
        for i in 0..num {
            let x = i as f32 * shift - centerx + j as f32 * 10.0;
            let y = j as f32 * shift + centery + 3.0;
            color += 1;

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 1.0),
                        custom_size: Some(Vec2::new(rad, rad)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(rad/2.0, rad/2.0, 0.0),
                    border_radius: None,
                });
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
