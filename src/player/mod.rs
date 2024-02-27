mod components;
mod systems;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use self::{
    components::PlayerAction,
    systems::{gravity, jump, move_hand, rise, run, spawn_player},
};
/// Adds functionality relating to the player character, including spawning them at the start of
/// the game and handling their movement.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (run, gravity, jump, rise, move_hand));
    }
}
