use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::components::{Jump, Player, PlayerAction, PlayerBundle};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = commands
        .spawn((
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
        ))
        .id();

    let hand = commands
        .spawn((PbrBundle {
            mesh: meshes.add(Cuboid::new(0.25, 0.25, 0.25)),
            material: materials.add(Color::rgb_u8(255, 255, 255)),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },))
        .id();
    commands.entity(player).push_children(&[hand]);
}

pub fn gravity(mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    for mut player in query.iter_mut() {
        player.translation = match player.translation {
            Some(vec) => Some(Vec3::new(vec.x, vec.y - 0.01, vec.z)),
            None => Some(Vec3::new(0.0, -0.1, 0.0)),
        };
    }
}

pub fn run(
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

pub fn jump(
    mut commands: Commands,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &KinematicCharacterControllerOutput,
            Entity,
        ),
        Without<Jump>,
    >,
) {
    if let Ok((action_state, output, entity)) = query.get_single_mut() {
        if action_state.pressed(&PlayerAction::Jump) && output.grounded {
            commands
                .entity(entity)
                .insert(Jump(Player::DEFAULT_JUMP_HEIGHT));
        }
    }
}

pub fn rise(
    mut commands: Commands,
    mut query: Query<(&mut KinematicCharacterController, &Jump, &Transform, Entity)>,
    time: Res<Time>,
) {
    if let Ok((mut controller, jump, transform, entity)) = query.get_single_mut() {
        let upward_movement = Player::DEFAULT_JUMP_SPEED * time.delta_seconds();

        if transform.translation.y + upward_movement >= jump.0 {
            commands.entity(entity).remove::<Jump>();
        }

        controller.translation = match controller.translation {
            Some(vec) => Some(Vec3::new(vec.x, vec.y + upward_movement, vec.z)),
            None => Some(Vec3::new(0.0, upward_movement, 0.0)),
        }
    };
}
