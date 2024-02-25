use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, movement);
    }
}
#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;

        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Right, KeyCode::KeyD);

        input_map.insert(Jump, KeyCode::KeyW);

        input_map
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PlayerBundle {
            player: Player,
            input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
        },
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            translation: Some(Vec3::new(0.0, 15.0, 0.0)),
            ..default()
        },
    ));
}

fn movement(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
) {
    let (action_state, mut character_controller) = query.single_mut();
    let mut direction_vector = Vec3::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                println!("direction: {:?}", direction);
                direction_vector.x += direction.x;
            }
        }
    }

    if action_state.just_pressed(&PlayerAction::Jump) {
        println!("Jump");
        direction_vector.y += 1.0;
    }

    //  gravity
    direction_vector.y -= 0.1;
    if let Ok(direction) = Direction3d::new(direction_vector) {
        character_controller.translation = Some(*direction);
    }
}
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    Left,
    Right,
    Jump,
}

impl PlayerAction {
    const DIRECTIONS: [Self; 2] = [PlayerAction::Left, PlayerAction::Right];

    fn direction(self) -> Option<Direction2d> {
        match self {
            PlayerAction::Left => Some(Direction2d::NEG_X),
            PlayerAction::Right => Some(Direction2d::X),
            _ => None,
        }
    }
}
