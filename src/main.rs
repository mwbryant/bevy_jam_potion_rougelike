use bevy::{render::texture::ImageSettings, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use prelude::{health::HealthPlugin, inventory::InventoryPlugin, *};

pub const HEIGHT: f32 = 700.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod animation;
mod enemy;
mod health;
mod ingredients;
mod input;
mod inventory;
mod map;
mod mouse;
mod music;
mod player;
mod prelude;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(texture_atlas(
        tile_size_x = 1.,
        tile_size_y = 1.,
        columns = 1,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "white_pixel.png")]
    player: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 67.,
        tile_size_y = 67.,
        columns = 4,
        rows = 2,
        padding_x = 1.,
        padding_y = 1.
    ))]
    #[asset(path = "Frog.png")]
    frog: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 70.,
        tile_size_y = 48.,
        columns = 8,
        rows = 1,
        padding_x = 1.,
        padding_y = 1.
    ))]
    #[asset(path = "Bat.png")]
    bat: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 32.,
        columns = 11,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "Potions.png")]
    drops: Handle<TextureAtlas>,

    #[asset(path = "FrogEyes.png")]
    frog_eyes: Handle<Image>,
    #[asset(path = "FrogLungs.png")]
    frog_lungs: Handle<Image>,
    #[asset(path = "FrogLegs.png")]
    frog_legs: Handle<Image>,
    #[asset(path = "BatEyes.png")]
    bat_eyes: Handle<Image>,
    #[asset(path = "BatWings.png")]
    bat_wings: Handle<Image>,
    #[asset(path = "BatEars.png")]
    bat_ears: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b0c060").unwrap()))
        .insert_resource(ImageSettings::default_nearest())
        .add_state(GameState::Splash)
        .add_loading_state(
            LoadingState::new(GameState::Splash)
                .continue_to_state(GameState::Main)
                .with_collection::<GameAssets>(),
        )
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Potion Game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PhysicsPlugin::default())
        //Our Plugins
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(MusicPlugin)
        //One off weird systems
        .add_startup_system(spawn_camera)
        .insert_resource(MousePos::default())
        .add_system(mouse_position)
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_temp_walls))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_temp_walls(mut commands: Commands, assets: Res<GameAssets>) {
    //Left
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::GRAY,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(-600.0, 0.0, 0.0)
                .with_scale(Vec3::new(100., 1100., 1.0)),
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 550.0).extend(1.0),
            border_radius: None,
        })
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World))
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
    //Right
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::GRAY,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(600.0, 0.0, 0.0).with_scale(Vec3::new(100., 1100., 1.0)),
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 550.0).extend(1.0),
            border_radius: None,
        })
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World))
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
    //Top
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::GRAY,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(0.0, 350.0, 0.0).with_scale(Vec3::new(1100., 100., 1.0)),
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(550.0, 50.0).extend(1.0),
            border_radius: None,
        })
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World))
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
    //Bottom
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::GRAY,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(0.0, -350.0, 0.0)
                .with_scale(Vec3::new(1100., 100., 1.0)),
            ..default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(550.0, 50.0).extend(1.0),
            border_radius: None,
        })
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World))
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Static);
}
