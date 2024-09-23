use bevy::prelude::*;

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
           .add_systems(Update, Camera::move_camera);
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
          .insert(Camera);
    }

    pub fn move_camera(keys: Res<ButtonInput<KeyCode>>,
                       mut camera_query: Query<&mut Transform, With<Camera>>,
                      ) 
    {
        let mut camera = camera_query.single_mut();
        let mut direction = Vec3::ZERO;

        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyW => direction.z -= 1.0,
                KeyCode::KeyS => direction.z += 1.0,
                KeyCode::KeyA => direction.x -= 1.0,
                KeyCode::KeyD => direction.x += 1.0,
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
}

fn camera_jump(cam: &mut WorldQuery::Item) {

}
