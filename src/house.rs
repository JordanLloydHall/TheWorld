use crate::human::Human;

use super::loading::TextureAssets;
use super::physics::Layer;
use super::GameState;
use super::{actions::Actions, territory::Territory};
use bevy::{prelude::*, utils::Duration};
use heron::*;
pub struct HousePlugin;

#[derive(Component)]
pub struct House;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_house));
        // .add_system_set(
        //     SystemSet::on_update(GameState::Playing)
        //         .with_system(spawn_rabbit)
        //         .with_system(tick_burrow_timer),
        // );
    }
}

fn spawn_house(mut commands: Commands, textures: Res<TextureAssets>) {
    let transform = Transform::from_translation(Vec3::new(0., 0., 1.));
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.burrow.clone(),
            transform: transform.clone(),
            ..Default::default()
        })
        .insert(House)
        .insert(Timer::new(Duration::from_secs(10), true))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(16., 16., 0.),
            border_radius: None,
        })
        .with_children(|a| {
            // a.spawn_bundle(SpriteBundle {
            //     sprite: Sprite {
            //         color: Color::rgba(0.5, 0.5, 1.0, 0.1),
            //         custom_size: Some(Vec2::new(400., 400.)),
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // })
            // .insert(Territory)
            // .insert(RigidBody::Sensor)
            // .insert(CollisionShape::Cuboid {
            //     half_extends: Vec3::new(200., 200., 0.),
            //     border_radius: None,
            // })
            // .insert(
            //     CollisionLayers::none()
            //         .with_group(Layer::Territory)
            //         .with_masks(&[Layer::Human]),
            // );
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_masks(&[Layer::Human, Layer::World]),
        );

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.rabbit.clone(),
            transform: Transform::from_translation(
                transform.translation
                    + Vec3::new(
                        (rand::random::<f32>() - 0.5) * 50.,
                        (rand::random::<f32>() - 0.5) * 50.,
                        0.1,
                    ),
            ),
            ..Default::default()
        })
        .insert(Human(rand::random::<f32>() * std::f32::consts::PI, false))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 8. })
        .insert(Velocity::from_linear(Vec3::ZERO))
        .insert(RotationConstraints::lock())
        .insert(Timer::from_seconds(5., true))
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Human)
                .with_masks(&[Layer::Territory, Layer::World]),
        );
}

// fn spawn_rabbit(
//     mut commands: Commands,
//     textures: Res<TextureAssets>,
//     mut burrows: Query<(&Transform, &mut House)>,
// ) {
//     for (transform, mut burrow) in burrows.iter_mut() {
//         if burrow.state == BurrowState::Spawn {
//             commands
//                 .spawn_bundle(SpriteBundle {
//                     texture: textures.rabbit.clone(),
//                     transform: Transform::from_translation(
//                         transform.translation
//                             + Vec3::new(
//                                 (rand::random::<f32>() - 0.5) * 50.,
//                                 (rand::random::<f32>() - 0.5) * 50.,
//                                 0.1,
//                             ),
//                     ),
//                     ..Default::default()
//                 })
//                 .insert(Rabbit)
//                 .insert(RigidBody::Dynamic)
//                 .insert(CollisionShape::Sphere { radius: 8. });
//             burrow.state = BurrowState::Idle;
//             println!("Spawned Rabbit!");
//         }
//     }
// }

// fn tick_burrow_timer(time: Res<Time>, mut burrows: Query<(&mut Timer, &mut Burrow)>) {
//     for (mut timer, mut burrow) in burrows.iter_mut() {
//         timer.tick(time.delta());
//         if timer.finished() {
//             burrow.state = BurrowState::Spawn;
//         }
//     }
// }
