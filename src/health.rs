use crate::{inventory::spawn_inventory_ui, prelude::*};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub health: f32,
    pub flashing: bool,
    pub damage_flash_timer: Timer,
    pub damage_flash_times_per_hit: usize,
}
#[derive(Component)]
pub struct HealthUI(usize);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .add_system_set(
                SystemSet::on_enter(GameState::Main)
                    .with_system(spawn_health_ui.before(spawn_inventory_ui)),
            )
            .add_system_set(SystemSet::on_update(GameState::Main).with_system(update_health_ui))
            .add_system(sword_collision)
            .add_system_to_stage(CoreStage::PostUpdate, damage_flash)
            .add_system(enemy_collision);
    }
}
fn update_health_ui(
    mut hearts: Query<(&mut UiImage, &HealthUI)>,
    player: Query<&Health, With<Player>>,
    assets: Res<GameAssets>,
) {
    if let Ok(player) = player.get_single() {
        for (mut image, heart) in &mut hearts {
            if player.health < heart.0 as f32 {
                *image = assets.heart_empty.clone().into();
            } else {
                *image = assets.heart.clone().into();
            }
        }
    }
}

fn spawn_health_ui(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        //align_content: AlignContent::FlexEnd,
                        margin: UiRect::all(Val::Px(20.0)),
                        flex_wrap: FlexWrap::Wrap,
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    //color: Color::rgb(0.95, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 1..6 {
                        parent
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::FlexEnd,
                                    align_self: AlignSelf::FlexEnd,
                                    size: Size::new(Val::Px(1.8 * 32.0), Val::Px(1.8 * 32.0)),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                image: assets.heart.clone().into(),
                                ..default()
                            })
                            .insert(HealthUI(i));
                    }
                });
        });
}

fn damage_flash(mut health: Query<(&mut Health, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut health, mut sprite) in &mut health {
        if health.flashing {
            health.damage_flash_timer.tick(time.delta());
            let flash = (health.damage_flash_timer.percent()
                * health.damage_flash_times_per_hit as f32)
                .fract();

            if flash < 0.5 {
                sprite.color.set_a(0.0);
            } else {
                sprite.color.set_a(1.0);
            }

            if health.damage_flash_timer.just_finished() {
                health.flashing = false;
                sprite.color.set_a(1.0);
            }
        }
    }
}

//Ugh is there a better way
fn sword_collision(
    mut collision_events: EventReader<CollisionEvent>,
    //Gross
    sword: Query<&Sword>,
    mut enemies: Query<&mut Health, With<Enemy>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            if sword.contains(d1.rigid_body_entity()) {
                //ugh
                if let Ok(mut health) = enemies.get_mut(d2.rigid_body_entity()) {
                    let sword = sword.get(d1.rigid_body_entity()).unwrap();
                    if sword.active && !health.flashing {
                        health.flashing = true;
                        health.health -= sword.damage;
                    }
                }
            }
            if sword.contains(d2.rigid_body_entity()) {
                if let Ok(mut health) = enemies.get_mut(d1.rigid_body_entity()) {
                    let sword = sword.get(d2.rigid_body_entity()).unwrap();
                    if sword.active && !health.flashing {
                        health.flashing = true;
                        health.health -= sword.damage;
                    }
                }
            }
        }
    }
}

//Ugh is there a better way
fn enemy_collision(
    mut collision_events: EventReader<CollisionEvent>,
    enemies: Query<&AiStage, With<Enemy>>,
    mut players: Query<&mut Health, With<Player>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            if let Ok(stage) = enemies.get(d1.rigid_body_entity()) {
                if let Ok(mut health) = players.get_mut(d2.rigid_body_entity()) {
                    if !health.flashing && matches!(stage, AiStage::CoolDown(..)) {
                        health.flashing = true;
                        health.health -= 1.;
                    }
                }
            }
            //Ahh there needs to be a better way to try these pairs
            //Probably something functional but I can't think of it atm
            if let Ok(stage) = enemies.get(d2.rigid_body_entity()) {
                if let Ok(mut health) = players.get_mut(d1.rigid_body_entity()) {
                    if !health.flashing && matches!(stage, AiStage::CoolDown(..)) {
                        health.flashing = true;
                        health.health -= 1.;
                    }
                }
            }
        }
    }
}
