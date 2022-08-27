use crate::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Ingredient>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_pickup_ingredient);
    }
}

fn player_pickup_ingredient(
    mut commands: Commands,
    mut player: Query<&mut Inventory, With<Player>>,
    mut drops: Query<(Entity, &Ingredient), Without<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            if let Ok(mut inventory) = player.get_mut(d1.rigid_body_entity()) {
                if let Ok((ent, ingredients)) = drops.get_mut(d2.rigid_body_entity()) {
                    commands.entity(ent).despawn_recursive();
                    inventory.items.push(*ingredients);
                }
            }
        }
    }
}
