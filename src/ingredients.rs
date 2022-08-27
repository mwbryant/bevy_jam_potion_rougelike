use crate::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Ingredient {
    FrogEyes,
    BatWings,
}

pub fn spawn_drop(commands: &mut Commands, to_spawn: Ingredient) {}
