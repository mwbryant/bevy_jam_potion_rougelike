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
        padding_x = 2.,
        padding_y = 2.
    ))]
    #[asset(path = "Witchbrew-tileset.png")]
    tileset: Handle<TextureAtlas>,
    #[asset(path = "Backgrounds/Witchbrew-Cross.png")]
    cross: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-EElbow.png")]
    eelbow: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-EEnd.png")]
    eend: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-Empty.png")]
    empty: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-EPipe.png")]
    epipe: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-ETee.png")]
    etee: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-NElbow.png")]
    nelbow: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-NEnd.png")]
    nend: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-Npipe.png")]
    npipe: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-NTee.png")]
    ntee: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-SElbow.png")]
    selbow: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-SEnd.png")]
    send: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-STee.png")]
    stee: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-WElbow.png")]
    weblow: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-WEnd.png")]
    wend: Handle<Image>,
    #[asset(path = "Backgrounds/Witchbrew-WTee.png")]
    wtee: Handle<Image>,
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
        //.insert_resource(WorldInspectorParams {
        //despawnable_entities: true,
        //..Default::default()
        //})
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
        .add_plugin(MapPlugin)
        //One off weird systems
        .add_startup_system(spawn_camera)
        .insert_resource(MousePos::default())
        .add_system(mouse_position)
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_room_exits))
        .add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_system(camera_follows_player.after(player_movement)),
        )
        .run();
}

fn camera_follows_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<Camera2d>, Without<Player>),
    >,
) {
    let player_transform = player_query.single().translation;
    let (mut camera_transform, ortho) = camera_query.single_mut();

    camera_transform.translation.x = player_transform.x;
    camera_transform.translation.y = player_transform.y;
    let max_cam = 31.0 * 0.8 * 64.0;
    if camera_transform.translation.y + ortho.top > max_cam {
        camera_transform.translation.y = max_cam - ortho.top;
    }
    if camera_transform.translation.y + ortho.bottom < -max_cam {
        camera_transform.translation.y = -max_cam - ortho.bottom;
    }
    if camera_transform.translation.x + ortho.right > max_cam {
        camera_transform.translation.x = max_cam - ortho.right;
    }
    if camera_transform.translation.x + ortho.left < -max_cam {
        camera_transform.translation.x = -max_cam - ortho.left;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_room_exits(mut commands: Commands, assets: Res<GameAssets>) {
    let exit = 31.0 * 0.8 * 64.0;
    //Left
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(-exit, 0.0, 0.0).with_scale(Vec3::new(100., 1100., 1.0)),
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
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(exit, 0.0, 0.0).with_scale(Vec3::new(100., 1100., 1.0)),
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
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, exit, 0.0).with_scale(Vec3::new(1100., 100., 1.0)),
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
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, -exit, 0.0).with_scale(Vec3::new(1100., 100., 1.0)),
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
