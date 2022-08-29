use crate::prelude::*;

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_start_menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(start_button));
    }
}
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn start_button(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Main).unwrap();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct MainMenu;

fn despawn_menu(mut commands: Commands, items: Query<Entity, With<MainMenu>>) {
    for ent in &items {
        commands.entity(ent).despawn_recursive();
    }
}

fn spawn_start_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::GREEN.into(),
            ..default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "WitchBrew",
                TextStyle {
                    font: assets.load("Font/DancingScript-VariableFont_wght.ttf"),
                    font_size: 128.0,
                    color: Color::rgb(0.2, 0.2, 0.2),
                },
            ));
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        margin: UiRect::all(Val::Px(20.0)),
                        size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                        ..default()
                    },
                    color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font: assets.load("Font/DancingScript-VariableFont_wght.ttf"),
                            font_size: 60.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent.spawn_bundle(TextBundle::from_section(
                "Code By: LogicProjects, Cathanos\nArt By: Madeline Hunt\nMusic By: Dean Manring",
                TextStyle {
                    font: assets.load("Font/DancingScript-VariableFont_wght.ttf"),
                    font_size: 48.0,
                    color: Color::rgb(0.2, 0.2, 0.2),
                },
            ));
        });
}
