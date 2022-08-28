use crate::prelude::*;

pub struct AnimationPlugin;

#[derive(Component)]
pub struct Animation {
    pub current_frame: usize,
    pub timer: Timer,
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_frog).add_system(animate_bat);
    }
}
fn animate_frog(
    mut frogs: Query<(
        &mut TextureAtlasSprite,
        &AiStage,
        &mut Animation,
        &EnemyType,
        &GlobalTransform,
    )>,
    player: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player) = player.get_single() {
        for (mut sprite, stage, mut animation, enemy, transform) in &mut frogs {
            if !matches!(enemy, EnemyType::Frog) {
                continue;
            }
            sprite.flip_x = player.translation().x - transform.translation().x > 1.0;

            match stage.clone() {
                AiStage::Jumping(_) => {
                    sprite.index = 5;
                }
                AiStage::Charge(_) => {
                    animation.timer.tick(time.delta());
                    if animation.timer.just_finished() {
                        if sprite.index == 3 {
                            sprite.index = 4;
                        } else {
                            sprite.index = 3;
                        }
                    }
                }
                AiStage::Dieing(_) => {
                    sprite.index = 7;
                }
                AiStage::Attack(_) => {
                    if sprite.index == 2 || sprite.index == 6 {
                        animation.timer.tick(time.delta());
                        if animation.timer.just_finished() {
                            sprite.index = 6;
                        }
                    } else {
                        sprite.index = 2;
                    }
                }
                _ => {
                    animation.timer.tick(time.delta());
                    if animation.timer.just_finished() {
                        if sprite.index == 0 {
                            sprite.index = 1;
                        } else {
                            sprite.index = 0;
                        }
                    }
                }
            }
        }
    }
}

fn animate_bat(
    mut bats: Query<(
        &mut TextureAtlasSprite,
        &AiStage,
        &mut Animation,
        &EnemyType,
        &GlobalTransform,
    )>,
    player: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player) = player.get_single() {
        for (mut sprite, stage, mut animation, enemy, transform) in &mut bats {
            if !matches!(enemy, EnemyType::Bat) {
                continue;
            }
            sprite.flip_x = player.translation().x - transform.translation().x > 1.0;

            match stage.clone() {
                AiStage::Charge(_) => {
                    animation.timer.tick(time.delta());
                    if animation.timer.just_finished() {
                        if sprite.index == 4 {
                            sprite.index = 5;
                        } else if sprite.index == 5 {
                            sprite.index = 6;
                        } else {
                            sprite.index = 4;
                        }
                    }
                }
                AiStage::Dieing(_) => {
                    sprite.index = 7;
                }
                AiStage::Attack(_) => {
                    sprite.index = 6;
                }
                _ => {
                    animation.timer.tick(time.delta());
                    if animation.timer.just_finished() {
                        if sprite.index == 0 {
                            sprite.index = 1;
                        } else if sprite.index == 1 {
                            sprite.index = 2;
                        } else {
                            sprite.index = 0;
                        }
                    }
                }
            }
        }
    }
}
