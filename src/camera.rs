use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const CAMERA_SPEED: f32 = 2.0;

/// # CameraPlugin
/// Extremely basic plugin for player manipulation
/// ## Game controls:
/// - W: moves camera along +z
/// - S: moves camera along -z
/// - A: moves camera along +x
/// - D: moves camera along -x
/// - Space: jumps
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Camera::setup_camera)
           .add_system(Camera::move_camera)
           .add_system(Camera::camera_velocity_shrink);
    }
}

#[derive(Component)]
pub struct Camera;

impl Camera {
    pub fn setup_camera(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(10.0, 12.0, 16.0)
                                     .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            Velocity(Vec3::ZERO),
            Camera,
        ));
    }

    pub fn move_camera(
        keys: Res<Input<KeyCode>>,
        mut camera_query: Query<&mut Velocity, With<Camera>>,
    ) {
        let mut vel = camera_query.single_mut().unwrap();

        let mut direction = Vec3::ZERO;
        
        for key in keys.get_pressed() {
            match key {
                KeyCode::W => direction.z -= 1.0,
                KeyCode::S => direction.z += 1.0,
                KeyCode::A => direction.x -= 1.0,
                KeyCode::D => direction.x += 1.0,
                KeyCode::Space => camera_jump(&mut vel),
                _ => {},
            }
        }

        // Normalize direction and apply speed
        if direction != Vec3::ZERO {
            let normalized_direction = direction.normalize() * CAMERA_SPEED;
            vel.0 += normalized_direction; // Update velocity
        }
    }

    pub fn camera_velocity_shrink(
        mut camera_query: Query<(&mut Velocity, &Transform), With<Camera>>,
        rapier_context: Res<RapierContext>,
    ) {
        let (mut vel, transform) = camera_query.single_mut().unwrap();
        
        // Dampen the velocities
        vel.0.x *= 0.9;
        vel.0.z *= 0.9;

        // Check if the camera is touching the floor
        let position = transform.translation;
        let collider_half_height = 0.5; // Assuming a cuboid collider

        // Check for intersection with colliders below the entity
        let below = position.y - collider_half_height - 0.01; // Slight offset to check just below
        let entity = /* Get your entity here */;

        let intersecting = rapier_context.intersections_with(entity, &CollisionGroups::new(1, 1));

        for collider in intersecting {
            if collider.position.y >= below {
                // Set Y velocity to zero if on the ground
                vel.0.y = 0.0;
                break;
            }
        }
    }
}

fn camera_jump(vel: &mut Velocity) {
    // Jumping is only allowed if not already jumping
    if vel.0.y > 0.0 {
        return;
    }

    // Apply jump velocity
    vel.0.y += 5.0; // Adjust jump strength as needed
}
