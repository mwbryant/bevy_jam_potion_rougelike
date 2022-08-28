use strum_macros::EnumIter;

use crate::prelude::*;

#[derive(Component, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
pub enum Ingredient {
    FrogEyes,
    BatWings,
}

pub fn spawn_drop(
    commands: &mut Commands,
    to_spawn: Ingredient,
    location: Vec3,
    assets: &Res<GameAssets>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.drops.clone(),
            transform: Transform::from_translation(location).with_scale(Vec3::splat(0.05)),
            ..default()
        })
        .insert(to_spawn)
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Sensor)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Ingredients))
        .insert(Damping::from_linear(10.5).with_angular(0.2));
}
