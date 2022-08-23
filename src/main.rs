use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use prelude::*;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod input;
mod mouse;
mod player;
mod prelude;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    Splash,
    Main,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
}

#[derive(AssetCollection)]
struct GameAssets {
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
        tile_size_x = 1.,
        tile_size_y = 1.,
        columns = 1,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "white_pixel.png")]
    enemy: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b0c060").unwrap()))
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
        .register_type::<Enemy>()
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_starting_scene))
        .add_system(enemy_movement)
        .insert_resource(MousePos::default())
        .add_system(mouse_position)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn enemy_movement(
    mut enemy: Query<(&Enemy, &mut Transform), Without<Player>>,
    player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player) = player.get_single() {
        for (enemy, mut transform) in &mut enemy {
            let direction = (player.translation - transform.translation).normalize();
            transform.translation += direction * enemy.speed * time.delta_seconds();
        }
    }
}

fn spawn_starting_scene(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::PURPLE,
                ..default()
            },
            texture_atlas: assets.enemy.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 0.0).with_scale(Vec3::splat(100.)),
            ..default()
        })
        .insert(Enemy { speed: 40.0 })
        //.insert(CollisionShape::Cuboid {
        //half_extends: Vec2::new(50.0, 50.0).extend(1.0),
        //border_radius: None,
        //})
        //.insert(RotationConstraints::lock())
        //.insert(RigidBody::Dynamic)
        .insert(Name::new("Enemy"));
}
