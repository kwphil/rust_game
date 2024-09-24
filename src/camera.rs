use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const CAMERA_SPEED: f32 = 2.0;
const JUMP_STRENGTH: f32 = 5.0;

/// # CameraPlugin
/// Extremely basic plugin for player manipulation
/// ## Game controls:
/// - W: moves camera along +z
/// - S: moves camera along -z
/// - A: moves camera along +x
/// - D: moves camera along -x
/// - Space: jumps

#[derive(Component)]
pub struct Camera;

impl Camera {
    pub fn setup_camera(mut commands: Commands) {
        commands.spawn(RigidBody::Dynamic).insert(
            Camera3dBundle {
                transform: Transform::from_xyz(10.0, 12.0, 16.0)
                                     .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            }
            ..Default::default()
        ).insert(Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        }).insert(Camera)
          .insert(Character::new());
    }
}

fn camera_jump(vel: &mut Vec3) {
    // Jumping is only allowed if not already jumping
    if vel.y > 0.0 {
        return;
    }

    // Apply jump velocity
    vel.y += JUMP_STRENGTH;
}
