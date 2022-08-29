use bevy::{
    render::{render_resource::TextureFormat, texture::ImageSettings},
    window::PresentMode,
};
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
mod world_gen;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(texture_atlas(
        tile_size_x = 80.,
        tile_size_y = 96.,
        columns = 9,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "Witch.png")]
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

    #[asset(path = "Heart.png")]
    heart: Handle<Image>,
    #[asset(path = "Heart_Empty.png")]
    heart_empty: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct BackgroundAssets {
    #[asset(texture_atlas(
        tile_size_x = 64.,
        tile_size_y = 64.,
        columns = 7,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "Witchbrew-tileset.png")]
    tileset: Handle<TextureAtlas>,
    #[asset(path = "Backgrounds/Witchbrew-Cross.png")]
    cross: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b0c060").unwrap()))
        .insert_resource(ImageSettings::default_nearest())
        .add_state(GameState::Splash)
        .add_loading_state(
            LoadingState::new(GameState::Splash)
                .continue_to_state(GameState::Main)
                .with_collection::<GameAssets>()
                .with_collection::<BackgroundAssets>(),
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
        //.add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_temp_walls))
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_cross_room))
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_system(camera_follows_player.after(player_movement)),
        )
        .run();
}

fn camera_follows_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single().translation;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.x;
    camera_transform.translation.y = player_transform.y;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn color_to_tile_index(r: u8, g: u8, b: u8) -> usize {
    match (r, g, b) {
        (0, 154, 83) => 1,
        _ => 0, //_ => unreachable!("Unknown Color {:?}", (r, g, b)),
    }
}

fn spawn_cross_room(
    mut commands: Commands,
    assets: Res<BackgroundAssets>,
    images: Res<Assets<Image>>,
) {
    let image = images.get(&assets.cross.clone()).unwrap();
    assert!(image.texture_descriptor.format == TextureFormat::Rgba8UnormSrgb);

    let width = image.size().x as usize;
    let height = image.size().y as usize;

    let pixel_size = 1.5;
    let tile_size = 64.0;

    for y in 0..height {
        for x in 0..width {
            let index = 4 * (x + y * width);
            let r = image.data[index];
            let g = image.data[index + 1];
            let b = image.data[index + 2];
            let index = color_to_tile_index(r, g, b);
            let id = commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: index,
                        ..default()
                    },
                    texture_atlas: assets.tileset.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * tile_size * pixel_size,
                        y as f32 * tile_size * pixel_size,
                        0.0,
                    )
                    .with_scale(Vec3::splat(pixel_size)),
                    ..default()
                })
                .id();
            if index == 1 {
                commands
                    .entity(id)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec2::splat(tile_size * pixel_size / 2.0).extend(1.0),
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::World),
                    )
                    .insert(RotationConstraints::lock())
                    .insert(RigidBody::Static);
            }
        }
    }

    //commands.spawn_bundle(SpriteBundle {
    //texture: assets.cross.clone(),
    //transform: Transform::from_scale(Vec3::new(19.5, 12.0, 1.0)),
    //..default()
    //});
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
