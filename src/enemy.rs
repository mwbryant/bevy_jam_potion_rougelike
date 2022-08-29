use rand::seq::SliceRandom;

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
    wait_time: f32,
    jump_time: f32,
    cooldown_time: f32,
}

#[derive(Component)]
pub enum EnemyType {
    Frog,
    Bat,
    Turtle,
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
    Wait(Timer),
    Jumping(Timer),
    Charge(Timer),
    Attack(Timer),
    CoolDown(Timer),
    Dieing(Timer),
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            //Why doesn't this show up in the inspector ugh
            .register_type::<AiStage>()
            .add_system(enemy_movement)
            .add_system(enemy_attack)
            .add_system(enemy_hitbox_disable)
            // on update because it depends on the game assets being loaded
            .add_system_set(SystemSet::on_update(GameState::Main).with_system(enemies_die));
        //.add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_enemy));
    }
}
fn enemies_die(
    mut commands: Commands,
    mut enemy: Query<
        (
            Entity,
            &Health,
            Option<&Ingredient>,
            &GlobalTransform,
            &mut AiStage,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
    assets: Res<GameAssets>,
) {
    for (ent, health, drop, transform, mut ai_stage) in &mut enemy {
        if health.health <= 0.0 && !matches!(*ai_stage, AiStage::Dieing(..)) {
            *ai_stage = AiStage::Dieing(Timer::from_seconds(1.0, false));
        }
        if let AiStage::Dieing(mut timer) = ai_stage.clone() {
            timer.tick(time.delta());
            if timer.just_finished() {
                commands.entity(ent).despawn_recursive();
                if let Some(drop) = drop {
                    spawn_drop(&mut commands, *drop, transform.translation(), &assets);
                }
            }
            //ugh
            *ai_stage = AiStage::Dieing(timer);
        }
    }
}

pub fn spawn_boss(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut pos = pos;
    pos.z = 10.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.turtle.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.5)),
            ..default()
        })
        .insert(Enemy {
            speed: 20.0,
            attack_speed: 250.0,
            target_offset: 200.0,
            charge_time: 0.5,
            attack_time: 0.8,
            wait_time: 0.8,
            jump_time: 0.4,
            cooldown_time: 0.5,
        })
        .insert(Health {
            health: 140.,
            flashing: false,
            damage_flash_timer: Timer::from_seconds(0.6, true),
            damage_flash_times_per_hit: 5,
        })
        .insert(EnemyType::Turtle)
        .insert(Animation {
            current_frame: 0,
            timer: Timer::from_seconds(0.35, true),
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(130.0, 35.0, 1.0),
            border_radius: Some(20.0),
        })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Enemy))
        .insert(Damping::from_linear(10.5).with_angular(0.2))
        .insert(AiStage::GetInRange)
        .insert(RoomMember)
        .insert(Name::new("Turtle"));
}
pub fn spawn_bat(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let bat_drops = vec![
        Ingredient::BatWings,
        Ingredient::BatEyes,
        Ingredient::BatEars,
    ];
    //Bat
    let mut pos = pos;
    pos.z = 10.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.bat.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.5)),
            ..default()
        })
        .insert(Enemy {
            speed: 40.0,
            attack_speed: 550.0,
            target_offset: 350.0,
            charge_time: 0.5,
            attack_time: 0.8,
            wait_time: 0.8,
            jump_time: 0.4,
            cooldown_time: 0.5,
        })
        .insert(Health {
            health: 20.,
            flashing: false,
            damage_flash_timer: Timer::from_seconds(0.6, true),
            damage_flash_times_per_hit: 5,
        })
        .insert(EnemyType::Bat)
        .insert(Animation {
            current_frame: 0,
            timer: Timer::from_seconds(0.35, true),
        })
        .insert(*bat_drops.choose(&mut rand::thread_rng()).unwrap())
        .insert(CollisionShape::Sphere { radius: 50.0 })
        .insert(RotationConstraints::lock())
        .insert(RigidBody::Dynamic)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Enemy))
        .insert(Damping::from_linear(10.5).with_angular(0.2))
        .insert(AiStage::GetInRange)
        .insert(RoomMember)
        .insert(Name::new("Bat"));
}

pub fn spawn_frog(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let frog_drops = vec![
        Ingredient::FrogEyes,
        Ingredient::FrogLungs,
        Ingredient::FrogLegs,
    ];
    let mut pos = pos;
    pos.z = 10.0;
    //Frog
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { ..default() },
            texture_atlas: assets.frog.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.5)),
            ..default()
        })
        .insert(Enemy {
            speed: 140.0,
            attack_speed: 450.0,
            target_offset: 150.0,
            charge_time: 1.0,
            attack_time: 0.4,
            wait_time: 0.8,
            jump_time: 0.4,
            cooldown_time: 0.5,
        })
        .insert(Health {
            health: 30.,
            flashing: false,
            damage_flash_timer: Timer::from_seconds(0.6, true),
            damage_flash_times_per_hit: 5,
        })
        .insert(EnemyType::Frog)
        .insert(Animation {
            current_frame: 0,
            timer: Timer::from_seconds(0.35, true),
        })
        .insert(*frog_drops.choose(&mut rand::thread_rng()).unwrap())
        .insert(CollisionShape::Sphere { radius: 50.0 })
        .insert(RotationConstraints::lock())
        .insert(RoomMember)
        .insert(RigidBody::Dynamic)
        .insert(CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Enemy))
        .insert(Damping::from_linear(10.5).with_angular(0.2))
        .insert(AiStage::Wait(Timer::from_seconds(0.8, false)))
        .insert(Name::new("Frog"));
}

fn enemy_movement(
    mut enemy: Query<(&Enemy, &mut AiStage, &mut Transform), Without<Player>>,
    player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const TOLERANCE: f32 = 1.0;
    if let Ok(player) = player.get_single() {
        for (enemy, mut stage, mut transform) in &mut enemy {
            //normal movement
            if matches!(*stage, AiStage::GetInRange) {
                let player_dir = (player.translation - transform.translation).normalize();
                let target = player.translation - player_dir * enemy.target_offset;
                let direction = target - transform.translation;
                if direction.length_squared() > TOLERANCE {
                    transform.translation +=
                        direction.normalize() * enemy.speed * time.delta_seconds();
                } else {
                    *stage = AiStage::Charge(Timer::from_seconds(enemy.charge_time, false));
                }
            }
            //Frog movement
            if let AiStage::Wait(mut timer) = stage.clone() {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *stage = AiStage::Jumping(Timer::from_seconds(enemy.jump_time, false));
                } else {
                    *stage = AiStage::Wait(timer.clone());
                }
            }
            if let AiStage::Jumping(mut timer) = stage.clone() {
                let player_dir = (player.translation - transform.translation).normalize();
                let target = player.translation - player_dir * enemy.target_offset;
                let direction = target - transform.translation;
                if direction.length_squared() > TOLERANCE {
                    transform.translation +=
                        direction.normalize() * enemy.speed * time.delta_seconds();
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        *stage = AiStage::Wait(Timer::from_seconds(enemy.wait_time, false));
                    } else {
                        *stage = AiStage::Jumping(timer.clone());
                    }
                } else {
                    *stage = AiStage::Charge(Timer::from_seconds(enemy.charge_time, false));
                }
            }
        }
    }
}

fn enemy_attack(
    mut enemy: Query<
        (
            &Enemy,
            &EnemyType,
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
        for (enemy, enemy_type, mut stage, mut transform, mut sprite) in &mut enemy {
            //clone here to make rust happy, idk why
            match stage.clone() {
                AiStage::GetInRange
                | AiStage::Dieing(..)
                | AiStage::Jumping(..)
                | AiStage::Wait(..) => continue,
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
                    let player_dir = (player.translation - transform.translation
                        + Vec3::new(3.0, 0.0, 0.0))
                    .normalize();
                    transform.translation +=
                        player_dir.normalize() * enemy.attack_speed * time.delta_seconds();
                    transform.translation.z = 10.0;
                }
                AiStage::CoolDown(mut timer) => {
                    sprite.color = Color::rgb(1.0, timer.percent(), timer.percent());
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        if matches!(enemy_type, EnemyType::Frog) {
                            *stage = AiStage::Wait(Timer::from_seconds(enemy.wait_time, false));
                        } else {
                            *stage = AiStage::GetInRange;
                        }
                    } else {
                        //Why do I need to reset this, rust pls
                        *stage = AiStage::CoolDown(timer);
                    }
                }
            }
        }
    }
}

fn enemy_hitbox_disable(mut enemy: Query<(&AiStage, &mut CollisionLayers, &mut RigidBody)>) {
    for (stage, mut collision, mut _rigid) in &mut enemy {
        if matches!(stage, AiStage::Attack(..)) {
            *collision = CollisionLayers::all_masks::<PhysicLayer>()
                .without_mask(PhysicLayer::Player)
                .with_group(PhysicLayer::Enemy);
        //*rigid = RigidBody::KinematicPositionBased;
        } else {
            *collision = CollisionLayers::all_masks::<PhysicLayer>().with_group(PhysicLayer::Enemy);
            //*rigid = (RigidBody::Dynamic);
        }
    }
}
