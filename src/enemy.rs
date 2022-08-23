use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use heron::rapier_plugin::nalgebra::default_allocator;

use crate::prelude::*;

pub struct EnemyPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
    attack_speed: f32,
    target_offset: f32,
    charge_time: f32,
    attack_time: f32,
    cooldown_time: f32,
}

//TODO should state transistions be impled on this or just let systems set it willy nilly
//TODO see if willy nilly is actually how that is spelled
//The flow here is the ai moves toward the player, once in range it starts winding up to hit
//Then the hit, then a cooldown, then it goes back to move toward the player
#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub enum AiStage {
    //Originally MoveToward but that was misleading because the AI always moves toward the player
    #[default]
    GetInRange,
    Charge(Timer),
    Attack(Timer),
    CoolDown(Timer),
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            //Why doesn't this show up in the inspector ugh
            .register_type::<AiStage>()
            .add_system(enemy_movement)
            .add_system(enemy_attack)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_enemy));
    }
}

fn spawn_enemy(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.enemy.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 0.0).with_scale(Vec3::splat(0.2)),
            ..default()
        })
        .insert(Enemy {
            speed: 40.0,
            attack_speed: 350.0,
            target_offset: 150.0,
            charge_time: 1.0,
            attack_time: 0.4,
            cooldown_time: 0.5,
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 50.0).extend(1.0),
            border_radius: None,
        })
        .insert(Health {
            health: 3,
            flashing: false,
            damage_flash_timer: Timer::from_seconds(1.0, true),
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Enemy))
        .insert(AiStage::GetInRange)
        .insert(Name::new("Enemy"));
}

fn enemy_movement(
    mut enemy: Query<(&Enemy, &mut AiStage, &mut Transform), Without<Player>>,
    player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const TOLERANCE: f32 = 1.0;
    if let Ok(player) = player.get_single() {
        for (enemy, mut stage, mut transform) in &mut enemy {
            let player_dir = (player.translation - transform.translation).normalize();
            let target = player.translation - player_dir * enemy.target_offset;
            let direction = target - transform.translation;
            if direction.length_squared() > TOLERANCE {
                transform.translation += direction.normalize() * enemy.speed * time.delta_seconds();
            } else if matches!(*stage, AiStage::GetInRange) {
                *stage = AiStage::Charge(Timer::from_seconds(enemy.charge_time, false));
            }
        }
    }
}

fn enemy_attack(
    mut enemy: Query<
        (
            &Enemy,
            &mut AiStage,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        Without<Player>,
    >,
    player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player) = player.get_single() {
        for (enemy, mut stage, mut transform, mut sprite) in &mut enemy {
            //clone here to make rust happy, idk why
            match stage.clone() {
                AiStage::GetInRange => continue,
                AiStage::Charge(mut timer) => {
                    sprite.color = Color::rgb(1.0, timer.percent_left(), timer.percent_left());

                    timer.tick(time.delta());
                    if timer.just_finished() {
                        *stage = AiStage::Attack(Timer::from_seconds(enemy.attack_time, false));
                    } else {
                        //Why do I need to reset this, rust pls
                        *stage = AiStage::Charge(timer);
                    }
                }
                AiStage::Attack(mut timer) => {
                    sprite.color = Color::rgb(1.0, 0.0, 0.0);

                    timer.tick(time.delta());
                    if timer.just_finished() {
                        *stage = AiStage::CoolDown(Timer::from_seconds(enemy.cooldown_time, false));
                    } else {
                        //Why do I need to reset this, rust pls
                        *stage = AiStage::Attack(timer);
                    }
                    let player_dir = (player.translation - transform.translation).normalize();
                    transform.translation +=
                        player_dir.normalize() * enemy.attack_speed * time.delta_seconds();
                }
                AiStage::CoolDown(mut timer) => {
                    sprite.color = Color::rgb(1.0, timer.percent(), timer.percent());
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        *stage = AiStage::GetInRange;
                    } else {
                        //Why do I need to reset this, rust pls
                        *stage = AiStage::CoolDown(timer);
                    }
                }
            }
        }
    }
}
