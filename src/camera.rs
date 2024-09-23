use bevy::prelude::*;

/// # CameraPlugin
/// Extremely basic plugin for camera manipulation
/// ## Game controls:
/// - W: moves camera along +z
/// - S: moves camera along -z
/// - A: moves camera along +x
/// - D: moves camera along -x
/// - R: moves camera along +y
/// - F: moves camera along -y
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
            Camera,
        ));
    }

    pub fn move_camera(keys: Res<ButtonInput<KeyCode>>,
                       mut camera_query: Query<&mut Transform, With<Camera>>,
                      ) 
    {
        let mut camera = camera_query.single_mut();

        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyA => camera.translation.x -= 5.0,
                KeyCode::KeyD => camera.translation.x += 5.0,
                KeyCode::KeyS => camera.translation.z -= 5.0,
                KeyCode::KeyW => camera.translation.z += 5.0,
                KeyCode::KeyR => camera.translation.y += 5.0,
                KeyCode::KeyF => camera.translation.y -= 5.0,
                _ => break,
            }
        }
    }
}
