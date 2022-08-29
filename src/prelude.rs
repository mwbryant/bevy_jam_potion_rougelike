pub use bevy::prelude::*;

pub use crate::animation::*;
pub use crate::enemy::*;
pub use crate::health::*;
pub use crate::ingredients::*;
pub use crate::ingredients::*;
pub use crate::input::*;
pub use crate::mouse::*;
pub use crate::music::*;
pub use crate::player::*;
pub use crate::world_gen::*;
pub use crate::*;

pub use heron::*;
pub use leafwing_input_manager::prelude::ActionState;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    Splash,
    Main,
}

#[derive(PhysicsLayer)]
pub enum PhysicLayer {
    World,
    Player,
    Sword,
    Enemy,
    Ingredients,
}
