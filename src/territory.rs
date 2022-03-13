use super::actions::Actions;
use super::loading::TextureAssets;
use super::GameState;
use bevy::prelude::*;
use heron::rapier_plugin::PhysicsWorld;

// pub struct TerritoryPlugin;

#[derive(Component)]
pub struct Territory;

// This plugin handles player related stuff like movement
// Player logic is only active during the State `GameState::Playing`
// impl Plugin for TerritoryPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource()
//     }
// }

// pub fn point_in_territory(physics_world: &PhysicsWorld, point: &Vec3) -> bool {
//     let result = physics_world.ray_cast_with_filter(
//         point,
//         point,
//         true,
//         targeter_transform.translation - world_center,
//         // Collision layers can be used to do group-based filtering on ray/shape-casts. See
//         // `layers.rs` example for more info. The default doesn't filter out any collisions.
//         CollisionLayers::none().with_group(Layer::Territory),
//         // We can also do fine-grained filtering using a closure. In this case, only shape cast to
//         // entities that don't have the `ShapeCastIgnored` component
//         |entity| ignored_entities.get(entity).is_err(),
//     );

//     todo!()
// }
