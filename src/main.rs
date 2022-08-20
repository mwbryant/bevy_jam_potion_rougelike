use std::{f32::consts::PI, time::Duration};

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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    speed: f32,
    roll_speed: f32,
    roll_direction: Vec3,
    rolling: bool,
    roll_timer: Timer,
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
        .register_type::<Player>()
        .add_plugin(InputPlugin)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_starting_scene))
        .add_system(player_movement)
        .add_system(player_dodge_roll)
        .add_system(enemy_movement)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
fn player_dodge_roll(
    mut player: Query<(&mut Player, &mut Transform, &ActionState<Action>)>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut transform, input)) = player.get_single_mut() {
        //Check input to init roll, also don't roll if no direction
        if !player.rolling {
            if input.just_pressed(Action::Roll) && player.roll_direction != Vec3::ZERO {
                player.rolling = true;
                player.roll_timer.set_elapsed(Duration::from_secs(0));
            } else {
                // Not rolling and not pressing the roll key
                return;
            }
        }

        //Apply roll movement
        transform.translation += player.roll_speed * player.roll_direction * time.delta_seconds();

        player.roll_timer.tick(time.delta());
        if player.roll_timer.just_finished() {
            player.rolling = false;
            transform.rotation = Quat::from_axis_angle(Vec3::Z, 0.0);
        } else {
            //TODO Probably replace with animation
            transform.rotation =
                Quat::from_axis_angle(Vec3::Z, 2.0 * PI * player.roll_timer.percent());
        }
    }
}

fn player_movement(
    mut player: Query<(&mut Player, &mut Transform, &ActionState<Action>)>,
    time: Res<Time>,
) {
    //I'd kinda perfer to crash if theres multiple players but adding a crash isn't that important
    if let Ok((mut player, mut transform, input)) = player.get_single_mut() {
        if player.rolling {
            //Movement locked during roll
            return;
        }
        // Track last movement for roll direction
        player.roll_direction = Vec3::ZERO;
        if input.pressed(Action::Forward) {
            transform.translation.y += time.delta_seconds() * player.speed;
            player.roll_direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if input.pressed(Action::Backward) {
            transform.translation.y -= time.delta_seconds() * player.speed;
            player.roll_direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if input.pressed(Action::Right) {
            transform.translation.x += time.delta_seconds() * player.speed;
            player.roll_direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if input.pressed(Action::Left) {
            transform.translation.x -= time.delta_seconds() * player.speed;
            player.roll_direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if player.roll_direction != Vec3::ZERO {
            player.roll_direction = player.roll_direction.normalize();
        }
    }
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
        .insert(Player {
            speed: 200.0,
            roll_speed: 900.0,
            roll_direction: Vec3::ZERO,
            rolling: false,
            roll_timer: Timer::from_seconds(0.4, true),
        })
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
