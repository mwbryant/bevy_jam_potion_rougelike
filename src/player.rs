use std::{collections::HashMap, f32::consts::PI, time::Duration};

use crate::{inventory::Inventory, prelude::*};
//use bevy::utils::HashMap;
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

pub struct PlayerPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub roll_speed: f32,
    pub roll_direction: Vec3,
    pub rolling: bool,
    pub roll_timer: Timer,
    pub swing_radius: f32,
    pub swing_dir_vec2: Vec2,
    pub swing_direction: f32,
    pub swinging: bool,
    pub swing_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Sword {
    pub active: bool,
    pub damage: f32,
}

#[derive(Component)]
pub struct SwordParent;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<Sword>()
            .add_system(player_movement)
            .add_system(sword_swing)
            .add_system(sword_updating)
            .add_system(player_dodge_roll)
            .add_system(player_hitbox_updating)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_player))
            .add_system_set(SystemSet::on_exit(GameState::Main).with_system(despawn_player));
    }
}

fn despawn_player(
    mut commands: Commands,
    player: Query<
        Entity,
        Or<(
            With<MainUI>,
            With<Player>,
            With<RoomMember>,
            With<ExitDirection>,
        )>,
    >,
) {
    for ent in &player {
        commands.entity(ent).despawn_recursive();
    }
}

fn player_hitbox_updating(mut player: Query<(&Player, &mut CollisionLayers, &mut RigidBody)>) {
    if let Ok((player, mut collision, mut _rigid)) = player.get_single_mut() {
        if player.rolling {
            *collision = CollisionLayers::all_masks::<PhysicLayer>()
                .without_mask(PhysicLayer::Enemy)
                .with_group(PhysicLayer::Player);
        //*rigid = RigidBody::KinematicPositionBased;
        } else {
            *collision =
                CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Player);
            //*rigid = (RigidBody::Dynamic);
        }
    }
}

fn sword_updating(player: Query<&Player>, mut sword: Query<(&mut Sword, &mut Visibility)>) {
    if let Ok(player) = player.get_single() {
        let (mut sword, mut sprite) = sword.single_mut();
        if player.swinging {
            sword.active = true;
            sprite.is_visible = true;
        } else {
            sword.active = false;
            sprite.is_visible = false;
        }
    }
}

//PERF this query could have a with marker to not be so broad
fn sword_swing(
    mut player: Query<(&Children, &mut Player, &ActionState<Action>)>,
    mut transforms: Query<(&mut Transform, &GlobalTransform), With<SwordParent>>,
    mouse: Res<MousePos>,
    time: Res<Time>,
) {
    for (children, mut player, action) in &mut player {
        // Handle starting swing
        if !player.swinging && action.just_pressed(Action::Swing) {
            player.swinging = true;
            player.swing_timer.set_elapsed(Duration::from_secs(0));
        }

        //If there are more than 1 child this needs rework
        //assert!(children.iter().count() == 1);
        for child in children {
            //If swinging tick timer and rotation depends on speed/timer percent
            if player.swinging {
                player.swing_timer.tick(time.delta());
                if player.swing_timer.just_finished() {
                    player.swinging = false;
                } else if let Ok((mut transform, _global)) = transforms.get_mut(*child) {
                    transform.rotation = Quat::from_axis_angle(
                        Vec3::Z,
                        player.swing_direction + player.swing_radius * player.swing_timer.percent(),
                    );
                }
            //Otherwise match mouse angle with a bit of an offset and record it
            } else if let Ok((mut transform, global)) = transforms.get_mut(*child) {
                let mut direction = **mouse - global.translation().truncate();
                if direction == Vec2::ZERO {
                    direction = Vec2::splat(0.001);
                }
                player.swing_dir_vec2 = direction;
                player.swing_direction =
                    -player.swing_radius / 2.0 - direction.angle_between(Vec2::Y);
                transform.rotation = Quat::from_axis_angle(Vec3::Z, player.swing_direction);
            }
        }
    }
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
            let flip = if player.roll_direction.x < 0.0 {
                1.0
            } else {
                -1.0
            };
            transform.rotation =
                Quat::from_axis_angle(Vec3::Z, flip * 2.0 * PI * player.roll_timer.percent());
        }
    }
}

pub fn player_movement(
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

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>, controls: Res<ControlSettings>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.player.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).with_scale(Vec3::splat(2.5)),
            ..default()
        })
        .insert(Inventory {
            items: HashMap::default(),
        })
        .insert(Player {
            speed: 200.0,
            roll_speed: 700.0,
            roll_direction: Vec3::ZERO,
            rolling: false,
            roll_timer: Timer::from_seconds(0.4, true),
            swing_radius: 1.5 * PI / 2.0,
            swing_direction: 0.0,
            swing_dir_vec2: Vec2::splat(0.0),
            swinging: false,
            swing_timer: Timer::from_seconds(0.35, true),
        })
        .insert(Animation {
            current_frame: 0,
            timer: Timer::from_seconds(0.15, true),
        })
        .insert(Health {
            health: 3.,
            flashing: false,
            damage_flash_timer: Timer::from_seconds(1.0, true),
            damage_flash_times_per_hit: 5,
        })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: controls.input.clone(),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(CollisionShape::Capsule {
            half_segment: 55.0,
            radius: 35.0,
        })
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Player))
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic)
        .insert(Damping::from_linear(20.5).with_angular(0.2))
        .with_children(|commands| {
            commands
                .spawn_bundle(SpatialBundle::default())
                .insert(Name::new("SwordParent"))
                .insert(SwordParent)
                .with_children(|commands| {
                    commands
                        .spawn_bundle(SpatialBundle {
                            //sprite: TextureAtlasSprite {
                            //color: Color::RED,
                            //..default()
                            //},
                            //texture_atlas: assets.player.clone(),
                            transform: Transform::from_xyz(0.0, 24.85, 0.1)
                                .with_scale(Vec3::new(0.2, 0.9, 1.0)),
                            //.with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4.0)),
                            ..default()
                        })
                        .insert(CollisionShape::Cuboid {
                            half_extends: Vec2::new(10.0, 95.0).extend(1.0),
                            border_radius: None,
                        })
                        .insert(RigidBody::Sensor)
                        .insert(
                            CollisionLayers::all_masks::<PhysicLayer>()
                                .with_group(PhysicLayer::Sword),
                        )
                        .insert(Sword {
                            active: false,
                            damage: 10.0,
                        })
                        .insert(Name::new("Sword"));
                });
        });
}
