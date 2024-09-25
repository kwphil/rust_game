use bevy::prelude::*;

pub fn move_char(
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
            KeyCode::Space => {
                // Jumping is only allowed if not already jumping
                if vel.y != 0.0 {
                    return;
                }

                // Apply jump velocity
                vel.y += JUMP_STRENGTH;
            },
            _ => {},
        }
    }

    // Normalize direction and apply speed
    if direction != Vec3::ZERO {
        let normalized_direction = direction.normalize() * CAMERA_SPEED;
        vel.linvel += normalized_direction; // Update velocity
    }
}