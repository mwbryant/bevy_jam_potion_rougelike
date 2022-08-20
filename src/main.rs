use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use input::{Action, ControlSettings, InputPlugin};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    Splash,
    Main,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
}

mod input;

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
        .add_plugin(InputPlugin)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_starting_scene))
        .add_system(player_movement)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn player_movement(
    mut player: Query<(&Player, &mut Transform, &ActionState<Action>)>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform, input)) = player.get_single_mut() {
        if input.pressed(Action::Forward) {
            transform.translation.y += time.delta_seconds() * player.speed;
        }
        if input.pressed(Action::Backward) {
            transform.translation.y -= time.delta_seconds() * player.speed;
        }
        if input.pressed(Action::Right) {
            transform.translation.x += time.delta_seconds() * player.speed;
        }
        if input.pressed(Action::Left) {
            transform.translation.x -= time.delta_seconds() * player.speed;
        }
    }
}

fn spawn_starting_scene(
    mut commands: Commands,
    assets: Res<GameAssets>,
    controls: Res<ControlSettings>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::GREEN,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(100.)),
            ..default()
        })
        .insert(Player { speed: 100.0 })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: controls.input.clone(),
            ..default()
        })
        .insert(Name::new("Player"));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::PURPLE,
                ..default()
            },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 0.0).with_scale(Vec3::splat(100.)),
            ..default()
        })
        .insert(Enemy { speed: 40.0 })
        .insert(Name::new("Enemy"));
}
