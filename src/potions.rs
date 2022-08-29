use bevy::prelude::*;

pub enum Potion{
    Speed(u16),
    Damage(u16),
    Health(u16),
    Reach(u16),
    Other(u16),
}

impl From<(Ingredient, Ingredient)> for Potion{
    fn From(ingredients: (Ingredient, Ingredient)) -> Potion{
        
    }
}
impl Potion{
    pub fn consume(potion: Potion, player: &mut Player, sword: &mut Sword, health: &mut Health) {
        match potion {
            Speed(strength) => {
                player.speed = player.speed * (1 + (0.3 * strength));
            },
            Damage(strength) => {
                sword.damage = sword.damage * (1 + (0.3 * strength));
            },
            Health(strength) =>{
                health = health + (1 * strength);
            },
            Reach(strength) =>{
                player.reach + (0.2 * strength)
            }
            _ => {}
        }
    }
}