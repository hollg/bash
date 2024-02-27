use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Marker component for the player character.
#[derive(Component)]
pub struct Player;

impl Player {
    pub const DEFAULT_RUN_SPEED: f32 = 7.0;
    pub const DEFAULT_JUMP_HEIGHT: f32 = 1.5;
    pub const DEFAULT_JUMP_SPEED: f32 = 5.5;
}

/// Bundle of components needed to spawn a player character. Includes input management.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;

        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Right, KeyCode::KeyD);

        input_map.insert(Jump, KeyCode::KeyW);

        input_map
    }
}

/// Component representing player's jump state. Contains the max height the jump should reach.
/// Component should be removed once the player's height reaches the max jump height.
#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Left,
    Right,
    Jump,
}

impl PlayerAction {
    pub const DIRECTIONS: [Self; 2] = [PlayerAction::Left, PlayerAction::Right];

    pub fn direction(self) -> Option<Direction2d> {
        match self {
            PlayerAction::Left => Some(Direction2d::NEG_X),
            PlayerAction::Right => Some(Direction2d::X),
            _ => None,
        }
    }
}
