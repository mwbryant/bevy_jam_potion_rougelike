use crate::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub health: usize,
    pub flashing: bool,
    pub damage_flash_timer: Timer,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .add_system(sword_collision)
            .add_system(enemy_collision);
    }
}

//Ugh is there a better way
fn sword_collision(
    mut collision_events: EventReader<CollisionEvent>,
    //Gross
    sword: Query<(), With<Sword>>,
    mut enemies: Query<&mut Health, With<Enemy>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            if sword.contains(d1.rigid_body_entity()) {
                unreachable!("I hope physics doesn't work this way");
            }
            if sword.contains(d2.rigid_body_entity()) {
                if let Ok(mut health) = enemies.get_mut(d1.rigid_body_entity()) {
                    if !health.flashing {
                        health.flashing = true;
                        health.health -= 1;
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
                    if !health.flashing && matches!(stage, AiStage::Attack(..)) {
                        health.flashing = true;
                        health.health -= 1;
                    }
                }
            }
            //Ahh there needs to be a better way to try these pairs
            //Probably something functional but I can't think of it atm
            if let Ok(stage) = enemies.get(d2.rigid_body_entity()) {
                if let Ok(mut health) = players.get_mut(d1.rigid_body_entity()) {
                    if !health.flashing && matches!(stage, AiStage::Attack(..)) {
                        health.flashing = true;
                        health.health -= 1;
                    }
                }
            }
        }
    }
}
