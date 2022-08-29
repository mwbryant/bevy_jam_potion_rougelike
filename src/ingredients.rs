use bevy_inspector_egui::Inspectable;
use strum_macros::EnumIter;

use crate::prelude::*;

#[derive(Component, Clone, Copy, EnumIter, Hash, PartialEq, Eq, Reflect, Inspectable, Default)]
pub enum Ingredient {
    #[default]
    FrogEyes,
    FrogLungs,
    FrogLegs,
    BatWings,
    BatEyes,
    BatEars,
}

impl Ingredient {
    pub fn to_sheet_index(&self) -> usize {
        match self {
            Ingredient::FrogEyes => 0,
            Ingredient::FrogLungs => 2,
            Ingredient::FrogLegs => 1,
            Ingredient::BatWings => 4,
            Ingredient::BatEyes => 3,
            Ingredient::BatEars => 5,
        }
    }
    pub fn to_sprite(&self, assets: &GameAssets) -> Handle<Image> {
        match self {
            Ingredient::FrogEyes => assets.frog_eyes.clone(),
            Ingredient::FrogLungs => assets.frog_lungs.clone(),
            Ingredient::FrogLegs => assets.frog_legs.clone(),
            Ingredient::BatEyes => assets.bat_eyes.clone(),
            Ingredient::BatWings => assets.bat_wings.clone(),
            Ingredient::BatEars => assets.bat_ears.clone(),
        }
    }
}

pub fn spawn_drop(
    commands: &mut Commands,
    to_spawn: Ingredient,
    location: Vec3,
    assets: &Res<GameAssets>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: to_spawn.to_sheet_index(),
                ..default()
            },
            texture_atlas: assets.drops.clone(),
            transform: Transform::from_translation(location).with_scale(Vec3::splat(2.5)),
            ..default()
        })
        .insert(to_spawn)
        .insert(CollisionShape::Sphere { radius: 30.0 })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Sensor)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Ingredients))
        .insert(Damping::from_linear(10.5).with_angular(0.2));
}
