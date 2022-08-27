use crate::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Ingredient>,
}

fn player_pickup_ingredient(
    mut commands: Commands,
    player: Query<(&mut Inventory, &Transform), With<Player>>,
    drops: Query<(&mut Ingredient, &Transform)>,
) {
}
