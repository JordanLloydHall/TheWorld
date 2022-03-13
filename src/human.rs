use crate::actions::Actions;
use crate::house::House;
use crate::loading::TextureAssets;
use crate::physics::{AddedObject, Layer};
use crate::territory::Territory;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::Duration;
use heron::prelude::*;
use heron::rapier_plugin::{PhysicsWorld, ShapeCastCollisionType};

pub struct HumanPlugin;

#[derive(Component, Debug)]
pub struct Human(pub f32, pub bool);

// This plugin handles player related stuff like movement
// Player logic is only active during the State `GameState::Playing`
impl Plugin for HumanPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_human)
                .with_system(place_house)
                .with_system(tick_human_timer),
        );
    }
}

fn move_human(mut commands: Commands, mut humans: Query<(&mut Velocity, &mut Human)>) {
    for (mut v, mut h) in humans.iter_mut() {
        h.0 += (rand::random::<f32>() - 0.5) * 0.1;
        let target_velocity =
            Velocity::from_linear(Vec3::new(10.0 * h.0.cos(), 10.0 * h.0.sin(), 0.0));

        v.linear = target_velocity.linear;
    }
}

fn tick_human_timer(time: Res<Time>, mut timers: Query<&mut Timer, With<Human>>) {
    for mut timer in timers.iter_mut() {
        timer.tick(time.delta());
    }
}

// fn human_in_territory(physics_world: PhysicsWorld, human: ) -> bool {
//     for event in events.iter() {
//         let (entity_1, entity_2) = event.rigid_body_entities();
//         let (layers_1, layers_2) = event.collision_layers();
//         // info!("{:?} {:?}", layers_1, layers_2);
//         let mut human = None;
//         if is_human(layers_1) && is_territory(layers_2) {
//             human = humans.get_mut(entity_1).ok();
//         } else if is_human(layers_2) && is_territory(layers_1) {
//             human = humans.get_mut(entity_2).ok();
//         }
//         if let Some(mut human) = human {
//             human.1 = match event {
//                 CollisionEvent::Started(_, _) => true,
//                 CollisionEvent::Stopped(_, _) => false,
//             };
//             // info!("{:?}", human);
//         }
//     }
// }

fn place_house(
    mut commands: Commands,
    physics_world: PhysicsWorld,
    textures: Res<TextureAssets>,
    mut added_object: ResMut<AddedObject>,
    mut humans: Query<(Entity, &Transform, &mut Timer, &Human)>,
) {
    for (entity, transform, mut timer, human) in humans.iter_mut() {
        if !added_object.0 && timer.finished() {
            let shape = CollisionShape::Cuboid {
                half_extends: Vec3::new(16., 16., 0.),
                border_radius: None,
            };
            let result = physics_world.ray_cast_with_filter(
                transform.translation,
                Vec3::new(50.0 * human.0.cos(), 50.0 * human.0.sin(), 0.0) + transform.translation,
                true,
                // Collision layers can be used to do group-based filtering on ray/shape-casts. See
                // `layers.rs` example for more info. The default doesn't filter out any collisions.
                CollisionLayers::new(Layer::Territory, Layer::Territory),
                // We can also do fine-grained filtering using a closure. In this case, only shape cast to
                // entities that don't have the `ShapeCastIgnored` component
                |_| true,
            );
            // if result.is_none() {
            //     continue;
            // }
            // dbg!(result);
            // w
            let result = physics_world.shape_cast_with_filter(
                &shape,
                transform.translation,
                Quat::IDENTITY,
                Vec3::new(50.0 * human.0.cos(), 50.0 * human.0.sin(), 0.0) + transform.translation,
                // Collision layers can be used to do group-based filtering on ray/shape-casts. See
                // `layers.rs` example for more info. The default doesn't filter out any collisions.
                CollisionLayers::new(Layer::World, Layer::World),
                // We can also do fine-grained filtering using a closure. In this case, only shape cast to
                // entities that don't have the `ShapeCastIgnored` component
                |e| true,
            );
            let mut new_transform = Transform::from_translation(
                Vec3::new(100.0 * human.0.cos(), 100.0 * human.0.sin(), 0.0)
                    + transform.translation,
            );
            if let Some(collision) = dbg!(result) {
                if let ShapeCastCollisionType::Collided(info) = collision.collision_type {
                    // Spawn a green block at the collision point
                    new_transform = Transform::from_translation(info.self_end_position);
                } else if let ShapeCastCollisionType::AlreadyPenetrating = collision.collision_type
                {
                    continue;
                }
            }
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.burrow.clone(),
                    transform: new_transform,
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
                        .with_masks(&[Layer::Territory, Layer::World, Layer::Human]),
                );
            timer.reset();
            added_object.0 = true;
        }
    }
}

fn is_human(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Human)
}

fn is_territory(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Territory)
}
