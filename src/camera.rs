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
           .add_systems(Update, Camera::move_camera)
           .add_systems(Update, Camera::camera_velocity_shrink);
    }
}

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
        }).insert(Camera);
    }

    pub fn move_camera(
        keys: Res<Input<KeyCode>>,
        mut query: Query<&mut Velocity, With<Camera>>,
    ) {
        let mut vel = query.single_mut();

        let mut direction = Vec3::ZERO;
        
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyW => direction.z -= 1.0,
                KeyCode::KeyS => direction.z += 1.0,
                KeyCode::KeyA => direction.x -= 1.0,
                KeyCode::KeyD => direction.x += 1.0,
                KeyCode::Space => camera_jump(&mut vel),
                _ => {},
            }
        }

        // Normalize direction and apply speed
        if direction != Vec3::ZERO {
            let normalized_direction = direction.normalize() * CAMERA_SPEED;
            vel.linvel += normalized_direction; // Update velocity
        }
    }

    // TODO: variable entity needs an entity on line 105
    pub fn camera_velocity_shrink(
        mut camera_query: Query<(&mut Velocity, &Transform), With<Camera>>,
        rapier_context: Res<RapierContext>,
    ) {
        let (mut vel, transform) = camera_query.single_mut();

        // Dummy for readability
        let dir = &mut vel;

        // Checking if velocities are exceeding what they need to be
        if dir.x > 1.0 {
            dir.x = 1.0;
        }

        if dir.x < -1.0 {
            dir.x = -1.0;
        }

        if dir.z > 1.0 {
            dir.z = 1.0;
        }

        if dir.z < -1.0 {
            dir.z = 1.0;
        }
        
        // Dampen the velocities
        dir.x *= 0.5;
        dir.z *= 0.5;

        // Check if the camera is touching the floor
        let position = transform.translation;
        let collider_half_height = 0.5; 

        // Check for intersection with colliders below the entity
        let below = position.y - collider_half_height - 0.01; // Slight offset to check just below
        let entity = /* Get your entity here */;

        let intersecting = rapier_context.intersections_with(entity, &CollisionGroups::new(1, 1));

        for collider in intersecting {
            if collider.position.y >= below {
                // Set Y velocity to zero if on the ground
                dir.y = 0.0;
                break;
            }
        }
    }
}

fn camera_jump(vel: &mut Velocity) {
    // Jumping is only allowed if not already jumping
    if vel.linvel.y > 0.0 {
        return;
    }

    // Apply jump velocity
    vel.linvel.y += 5.0; // Adjust jump strength as needed
}
