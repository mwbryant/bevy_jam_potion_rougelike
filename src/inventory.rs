use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Ingredient>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_inventory_ui))
            .add_system(player_pickup_ingredient);
    }
}

fn spawn_inventory_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
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
                        align_self: AlignSelf::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        margin: UiRect::all(Val::Px(20.0)),
                        //size: Size::new(Val::Px(200.0), Val::Percent(70.0)),
                        ..default()
                    },
                    color: Color::rgb(0.95, 0.15, 0.15).into(),
                    ..default()
                })
                //Item buttons
                .with_children(|parent| {
                    for ingredient in Ingredient::iter() {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    flex_direction: FlexDirection::RowReverse,
                                    align_items: AlignItems::FlexStart,
                                    size: Size::new(Val::Px(164.0), Val::Px(164.0)),
                                    margin: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                color: Color::rgb(0.5, 0.6, 0.9).into(),
                                ..default() //Count text
                            })
                            .insert(ingredient)
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "0",
                                        TextStyle {
                                            font: asset_server
                                                .load("Font/DancingScript-VariableFont_wght.ttf"),
                                            font_size: 36.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
        });
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
