use std::collections::HashMap;

//use bevy::utils::HashMap;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    pub items: HashMap<Ingredient, usize>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_inventory_ui))
            //.register_inspectable::<Inventory>()
            .add_system(update_inventory_ui)
            .add_system(player_pickup_ingredient);
    }
}

fn update_inventory_ui(
    buttons: Query<(&Children, &Ingredient), With<Button>>,
    //Gross pls let me recursively climb the heirarchy
    images: Query<&Children, With<UiImage>>,
    mut text: Query<&mut Text>,
    inventory: Query<&Inventory, With<Player>>,
) {
    if let Ok(inventory) = inventory.get_single() {
        for (children, ingredient) in &buttons {
            for child in children {
                if let Ok(children2) = images.get(*child) {
                    for child in children2 {
                        if let Ok(mut text) = text.get_mut(*child) {
                            let count = inventory.items.get(ingredient).unwrap_or(&0);
                            *text = Text::from_section(
                                format!("{}", count),
                                text.sections[0].style.clone(),
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_inventory_ui(
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
                        align_self: AlignSelf::FlexStart,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_content: AlignContent::FlexEnd,
                        margin: UiRect::all(Val::Px(20.0)),
                        flex_wrap: FlexWrap::Wrap,
                        size: Size::new(Val::Px(200.0), Val::Percent(30.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    //color: Color::rgb(0.95, 0.15, 0.15).into(),
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
                                    size: Size::new(Val::Px(2.5 * 32.0), Val::Px(2.5 * 32.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                color: Color::rgb(0.5, 0.6, 0.9).into(),
                                ..default() //Count text
                            })
                            .insert(ingredient)
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(ImageBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::RowReverse,
                                            align_items: AlignItems::FlexStart,
                                            size: Size::new(
                                                Val::Px(2.5 * 32.0),
                                                Val::Px(2.5 * 32.0),
                                            ),
                                            ..default()
                                        },
                                        image: ingredient.to_sprite(&assets).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(TextBundle {
                                            text: Text::from_section(
                                                "0",
                                                TextStyle {
                                                    font: asset_server.load(
                                                        "Font/DancingScript-VariableFont_wght.ttf",
                                                    ),
                                                    font_size: 20.0,
                                                    color: Color::BLACK,
                                                },
                                            ),
                                            style: Style {
                                                margin: UiRect::all(Val::Px(8.0)),
                                                ..default()
                                            },
                                            ..default()
                                        });
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
                    if inventory.items.contains_key(ingredients) {
                        let count = inventory.items[ingredients] + 1;
                        inventory.items.insert(*ingredients, count);
                    } else {
                        inventory.items.insert(*ingredients, 1);
                    }
                }
            }
        }
    }
}
