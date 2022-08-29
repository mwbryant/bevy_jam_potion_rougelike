use crate::prelude::*;

pub enum Potion {
    Speed(u16),
    Damage(u16),
    Health(u16),
    Other(u16),
}

impl Potion {
    pub fn new(ingredients: (Ingredient, Ingredient)) -> Potion {
        let bat_drops = vec![
            Ingredient::BatWings,
            Ingredient::BatEyes,
            Ingredient::BatEars,
        ];
        let frog_drops = vec![
            Ingredient::FrogEyes,
            Ingredient::FrogLungs,
            Ingredient::FrogLegs,
        ];
        if bat_drops.contains(&ingredients.0) && frog_drops.contains(&ingredients.1) {
            Potion::Health(1)
        } else if frog_drops.contains(&ingredients.0) && bat_drops.contains(&ingredients.1) {
            Potion::Health(1)
        } else {
            Potion::Speed(1)
        }
    }
    pub fn consume(&self, player: &mut Player, health: &mut Health) {
        match self {
            Potion::Speed(strength) => {
                player.speed = player.speed * (1.0 + (0.1 * *strength as f32));
            }
            //Potion::Damage(strength) => {
            //sword.damage = sword.damage * (1.0 + (0.3 * *strength as f32));
            //}
            Potion::Health(strength) => {
                health.health = health.health + (1 * strength) as f32;
            }
            _ => {}
        }
    }
}
