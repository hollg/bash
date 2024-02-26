use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (run, gravity, jump, rise));
    }
}
#[derive(Component)]
struct Player;

impl Player {
    const DEFAULT_RUN_SPEED: f32 = 7.0;
    const DEFAULT_JUMP_HEIGHT: f32 = 1.5;
    const DEFAULT_JUMP_SPEED: f32 = 5.5;
}

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
        KinematicCharacterController::default(),
    ));
}

fn gravity(mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    for mut player in query.iter_mut() {
        println!("gravity");
        player.translation = match player.translation {
            Some(vec) => Some(Vec3::new(vec.x, vec.y - 0.01, vec.z)),
            None => Some(Vec3::new(0.0, -0.1, 0.0)),
        };
    }
}

fn run(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (action_state, mut character_controller) = query.single_mut();
    let mut direction_vector = Vec3::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector.x +=
                    direction.x * Player::DEFAULT_RUN_SPEED * time.delta_seconds();
            }
        }
    }

    character_controller.translation = match character_controller.translation {
        Some(vec) => Some(Vec3::new(direction_vector.x, vec.y, vec.z)),
        None => Some(direction_vector),
    };
}

fn jump(mut commands: Commands, mut query: Query<(&ActionState<PlayerAction>, Entity)>) {
    let (action_state, entity) = query.single_mut();

    if action_state.pressed(&PlayerAction::Jump) {
        commands
            .entity(entity)
            .insert(Jump(Player::DEFAULT_JUMP_HEIGHT));
    }
}

fn rise(
    mut commands: Commands,
    mut query: Query<(&mut KinematicCharacterController, &Jump, &Transform, Entity)>,
    time: Res<Time>,
) {
    if let Ok((mut controller, jump, transform, entity)) = query.get_single_mut() {
        let upward_movement = Player::DEFAULT_JUMP_SPEED * time.delta_seconds();

        if transform.translation.y + upward_movement >= jump.0 {
            // remove jump component
            commands.entity(entity).remove::<Jump>();
        }
        controller.translation = match controller.translation {
            Some(vec) => Some(Vec3::new(vec.x, vec.y + upward_movement, vec.z)),
            None => Some(Vec3::new(0.0, upward_movement, 0.0)),
        }
    };
}

#[derive(Component)]
struct Jump(f32);

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
