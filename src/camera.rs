use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const __CAMERA_SPEED__: f32 = 2.0;
static JUMP: bool = false;

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
        app.add_systems(Startup, Camera::setup_camera)
           .add_systems(Update, Camera::move_camera)
           .add_systems(Update, Camera::camera_velocity_shrink);
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
            ..Default::default(),
        )).insert(Collider::cuboid(0.5, 0.5, 0.5))
          .insert(Veclocity(Vec::new(0.0, 0.0, 0.0))
          .insert(Camera);
    }

    pub fn move_camera(keys: Res<ButtonInput<KeyCode>>,
                       mut camera_query: Query<&mut Velocity, With<Camera>>,
                      ) 
    {
        let mut vel = &mut camera_query.single_mut();
        
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyW => vel.0.z -= 1.0,
                KeyCode::KeyS => vel.0.z += 1.0,
                KeyCode::KeyA => vel.0.x -= 1.0,
                KeyCode::KeyD => vel.0.x += 1.0,
                KeyCode::Space => camera_jump(&mut camera);
                _ => break,
            }
        }

        // Normalize direction and apply speed
        if direction != Vec3::ZERO {
            // Change speed at top
            direction = direction.normalize() * __CAMERA_SPEED__;
            transform.translation += direction;
        }
    }

    pub fn collision_detection(
        mut commands: Commands,
        events: Res<Events<CollisionEvent>>,
        mut reader: Local<EventReader<CollisionEvents>>,
    ) {
        for event in collision_reader.iter(&collision_events) {
            match *event {
                CollisionEvent::Started(_, _) => {
                    println!("Collision detected!");
                    // Handle collision logic here (e.g., prevent movement)
                }
                CollisionEvent::Stopped(_, _) => {
                    println!("Collision ended.");
                }
            }
        }
    }

    /// TODO: Update the jump mechanics.
    // Updates the velocities of the camera
    pub fn camera_velocity_shrink (
        mut camera_query: Query<(&mut Velocity, &mut Transform, &Camera)>,
        rapier_context: Res<RapierContext>,
    ) {
        let mut camera = camera_query.single_mut();
        let mut vel = &mut camera.0;
        let mut form = &mut camera.1;
        let mut cam = &mut camera.2;
        
        // Lower the velocities
        if vel.0.x < 0 {
            vel.0.x += 0.1;
        }

        if vel.0.x > 0 {
            vel.0.x -= 0.1;
        }

        if vel.0.z < 0 {
            vel.0.z += 0.1;
        }

        if vel.0.z > 0 {
            vel.0.z -= 0.1;
        }

        /// NEEDS WORK
        // Check if the camera is touching the floor
        let position = transform.translation;
        let collider_half_height = 0.5; // Assuming a cuboid collider

        // Check for intersection with colliders below the entity
        let below = position.y - collider_half_height - 0.01; // Slight offset to check just below
        let intersecting = rapier_context.intersections_with(entity, &CollisionGroups::new(1, 1));

        for collider in intersecting {
            if collider.position.y >= below {
                // Set Y velocity to zero if on the ground
                commands.entity(entity).insert(Velocity(Vec3::new(velocity.0.x, 0.0, velocity.0.z)));
                break;
            }
        }
        
        
    }
}

fn camera_jump(vel: &mut WorldQuery::Item) {
    if JUMP {
        return;
    }
        
    JUMP = true;

    vel.0.y += 1.0;
}
