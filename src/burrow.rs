use crate::loading::TextureAssets;
use crate::GameState;
use crate::{actions::Actions, rabbit::Rabbit};
use bevy::{prelude::*, utils::Duration};
use heron::*;

pub struct BurrowPlugin;

#[derive(Component)]
pub struct Burrow {
    state: BurrowState,
}

#[derive(PartialEq)]
pub enum BurrowState {
    Idle,
    Spawn,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for BurrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_burrow))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_rabbit)
                    .with_system(tick_burrow_timer),
            );
    }
}

fn spawn_burrow(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.burrow.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Burrow {
            state: BurrowState::Spawn,
        })
        .insert(Timer::new(Duration::from_secs(10), true))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere { radius: 16. });
}

fn spawn_rabbit(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut burrows: Query<(&Transform, &mut Burrow)>,
) {
    for (transform, mut burrow) in burrows.iter_mut() {
        if burrow.state == BurrowState::Spawn {
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
                .insert(Rabbit)
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Sphere { radius: 8. });
            burrow.state = BurrowState::Idle;
            println!("Spawned Rabbit!");
        }
    }
}

fn tick_burrow_timer(time: Res<Time>, mut burrows: Query<(&mut Timer, &mut Burrow)>) {
    for (mut timer, mut burrow) in burrows.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            burrow.state = BurrowState::Spawn;
        }
    }
}
