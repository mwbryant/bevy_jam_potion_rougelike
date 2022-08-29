use bevy::{
    ecs::system::SystemParam,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_inspector_egui::bevy_egui::{
    egui::{Align2, Area, Grid, Window},
    EguiContext,
};
use derive_more::Display;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

const UI_MARGIN: f32 = 10.0;

pub struct InputPlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Display)]
pub enum Action {
    // Movement
    Forward,
    Backward,
    Left,
    Right,
    Swing,
    Roll,
}

pub struct ControlSettings {
    active: bool,
    pub input: InputMap<Action>,
}

impl Default for ControlSettings {
    fn default() -> Self {
        let mut input = InputMap::default();
        input
            .insert(KeyCode::W, Action::Forward)
            .insert(KeyCode::S, Action::Backward)
            .insert(KeyCode::A, Action::Left)
            .insert(KeyCode::D, Action::Right)
            .insert(MouseButton::Left, Action::Swing)
            .insert(KeyCode::Space, Action::Roll);
        Self {
            active: false,
            input,
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .insert_resource(ControlSettings::default());
        //.add_system(controls_window_system);
        //.add_system(buttons_system)
        //.add_system(toggle_keybind_menu)
        //.add_system(binding_window_system);
    }
}

//TODO make this also a leafwing style input
fn toggle_keybind_menu(keyboard: Res<Input<KeyCode>>, mut settings: ResMut<ControlSettings>) {
    if keyboard.just_pressed(KeyCode::Grave) {
        settings.active = !settings.active;
    }
}

// Remapping system just taken from leafwing examples with some modifications
fn controls_window_system(
    mut commands: Commands,
    mut egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    control_settings: ResMut<ControlSettings>,
) {
    if !control_settings.active {
        return;
    }
    let main_window = windows.get_primary().unwrap();
    let window_width_margin = egui.ctx_mut().style().spacing.window_margin.left * 2.0;
    Window::new("Settings")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .default_width(main_window.width() - UI_MARGIN * 2.0 - window_width_margin)
        .show(egui.ctx_mut(), |ui| {
            const INPUT_VARIANTS: usize = 3;
            const COLUMNS_COUNT: usize = INPUT_VARIANTS + 1;
            Grid::new("Control grid")
                .num_columns(COLUMNS_COUNT)
                .striped(true)
                .min_col_width(ui.available_width() / COLUMNS_COUNT as f32 - window_width_margin)
                .show(ui, |ui| {
                    for action in Action::variants() {
                        ui.label(action.to_string());
                        let inputs = control_settings.input.get(action);
                        for index in 0..INPUT_VARIANTS {
                            let button_text = match inputs.get_at(index) {
                                Some(UserInput::Single(InputKind::GamepadButton(
                                    gamepad_button,
                                ))) => {
                                    format!("🎮 {:?}", gamepad_button)
                                }
                                Some(UserInput::Single(InputKind::Keyboard(keycode))) => {
                                    format!("🖮 {:?}", keycode)
                                }
                                Some(UserInput::Single(InputKind::Mouse(mouse_button))) => {
                                    format!("🖱 {:?}", mouse_button)
                                }
                                _ => "Empty".to_string(),
                            };
                            if ui.button(button_text).clicked() {
                                commands.insert_resource(ActiveBinding::new(action, index));
                            }
                        }
                        ui.end_row();
                    }
                });
            ui.expand_to_include_rect(ui.available_rect_before_wrap());
        });
}
fn buttons_system(
    mut egui: ResMut<EguiContext>,
    mut control_settings: ResMut<ControlSettings>,
    mut player_mappings: Query<&mut InputMap<Action>>,
) {
    if !control_settings.active {
        return;
    }
    Area::new("Settings buttons area")
        .anchor(Align2::RIGHT_BOTTOM, (-UI_MARGIN, -UI_MARGIN))
        .show(egui.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Restore defaults").clicked() {
                    *control_settings = ControlSettings::default();
                }
                if ui.button("Apply").clicked() {
                    *player_mappings.single_mut() = control_settings.input.clone();
                }
            })
        });
}
fn binding_window_system(
    mut commands: Commands,
    mut egui: ResMut<EguiContext>,
    mut input_events: InputEvents,
    active_binding: Option<ResMut<ActiveBinding>>,
    mut control_settings: ResMut<ControlSettings>,
) {
    let mut active_binding = match active_binding {
        Some(active_binding) => active_binding,
        None => return,
    };
    Window::new(format!("Binding \"{}\"", active_binding.action))
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx_mut(), |ui| {
            if let Some(conflict) = &active_binding.conflict {
                ui.label(format!(
                    "Input \"{}\" is already used by \"{}\"",
                    conflict.input_button, conflict.action
                ));
                ui.horizontal(|ui| {
                    if ui.button("Replace").clicked() {
                        control_settings
                            .input
                            .remove(conflict.action, conflict.input_button);
                        control_settings.input.insert_at(
                            conflict.input_button,
                            active_binding.action,
                            active_binding.index,
                        );
                        commands.remove_resource::<ActiveBinding>();
                    }
                    if ui.button("Cancel").clicked() {
                        commands.remove_resource::<ActiveBinding>();
                    }
                });
            } else {
                ui.label("Press any key now or Esc to cancel");
                if let Some(input_button) = input_events.input_button() {
                    let conflict_action =
                        control_settings.input.iter().find_map(|(inputs, action)| {
                            if action != active_binding.action
                                && inputs.contains(&input_button.into())
                            {
                                return Some(action);
                            }
                            None
                        });
                    if let Some(action) = conflict_action {
                        active_binding.conflict.replace(BindingConflict {
                            action,
                            input_button,
                        });
                    } else {
                        control_settings.input.insert_at(
                            input_button,
                            active_binding.action,
                            active_binding.index,
                        );
                        commands.remove_resource::<ActiveBinding>();
                    }
                }
            }
        });
}
struct ActiveBinding {
    action: Action,
    index: usize,
    conflict: Option<BindingConflict>,
}
impl ActiveBinding {
    fn new(action: Action, index: usize) -> Self {
        Self {
            action,
            index,
            conflict: None,
        }
    }
}
struct BindingConflict {
    action: Action,
    input_button: InputKind,
}
/// Helper for collecting input
#[derive(SystemParam)]
struct InputEvents<'w, 's> {
    keys: EventReader<'w, 's, KeyboardInput>,
    mouse_buttons: EventReader<'w, 's, MouseButtonInput>,
    gamepad_events: EventReader<'w, 's, GamepadEvent>,
}
impl InputEvents<'_, '_> {
    fn input_button(&mut self) -> Option<InputKind> {
        if let Some(keyboard_input) = self.keys.iter().next() {
            if keyboard_input.state == ButtonState::Released {
                if let Some(key_code) = keyboard_input.key_code {
                    return Some(key_code.into());
                }
            }
        }
        if let Some(mouse_input) = self.mouse_buttons.iter().next() {
            if mouse_input.state == ButtonState::Released {
                return Some(mouse_input.button.into());
            }
        }
        if let Some(GamepadEvent {
            gamepad: _,
            event_type,
        }) = self.gamepad_events.iter().next()
        {
            if let GamepadEventType::ButtonChanged(button, strength) = event_type.to_owned() {
                if strength <= 0.5 {
                    return Some(button.into());
                }
            }
        }
        None
    }
}
