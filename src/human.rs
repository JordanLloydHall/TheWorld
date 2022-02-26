use crate::actions::Actions;
use crate::house::House;
use crate::loading::TextureAssets;
use crate::physics::Layer;
use crate::territory::Territory;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::Duration;
use heron::prelude::*;

pub struct HumanPlugin;

#[derive(Component)]
pub struct Human(pub f32, pub bool);

// This plugin handles player related stuff like movement
// Player logic is only active during the State `GameState::Playing`
impl Plugin for HumanPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_human)
                .with_system(place_house)
                .with_system(tick_human_timer)
                .with_system(human_in_territory),
        );
    }
}

fn move_human(mut commands: Commands, mut humans: Query<(&mut Velocity, &mut Human)>) {
    for (mut v, mut h) in humans.iter_mut() {
        h.0 += (rand::random::<f32>() - 0.5) * 0.1;
        let target_velocity =
            Velocity::from_linear(Vec3::new(25.0 * h.0.cos(), 25.0 * h.0.sin(), 0.0));

        v.linear = target_velocity.linear;
    }
}

fn tick_human_timer(time: Res<Time>, mut timers: Query<&mut Timer, With<Human>>) {
    for mut timer in timers.iter_mut() {
        timer.tick(time.delta());
    }
}

fn human_in_territory(mut events: EventReader<CollisionEvent>, mut humans: Query<&mut Human>) {
    for event in events.iter() {
        let (entity_1, _) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();
        info!("{:?} {:?}", layers_1, layers_2);
        if is_human(layers_1) && is_territory(layers_2) {
            let mut human = humans.get_mut(entity_1).unwrap();
            match event {
                CollisionEvent::Started(_, _) => {
                    info!("Human started colliding with Territory");
                    human.1 = true;
                }
                CollisionEvent::Stopped(_, _) => {
                    info!("Human stopped colliding with Territory");
                    human.1 = false;
                }
            }
        }
    }
}

fn place_house(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut humans: Query<(&Transform, &mut Timer, &Human)>,
) {
    for (transform, mut timer, human) in humans.iter_mut() {
        if timer.finished() && human.1 {
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
                    a.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.5, 0.5, 1.0, 0.1),
                            custom_size: Some(Vec2::new(400., 400.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Territory)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(200., 200., 0.),
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::Territory)
                            .with_masks(&[Layer::Human]),
                    );
                });

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
                .insert(Timer::from_seconds(10., true))
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::Human)
                        .with_masks(&[Layer::Territory, Layer::World]),
                );
            timer.reset();
        }
    }
}

fn is_human(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Human)
}

fn is_territory(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Territory)
}
