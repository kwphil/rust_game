use bevy::prelude::*; 
use crate::*;

mod player;
mod camera;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::Camera::setup_camera)
           .add_systems(Update, (velocity_shrink, player::move_char));
    }
}

#[derive(Component)]
pub struct Character {
  pub name: String,
  pub health: u32,
  pub weapons: Vec<weapon::Weapon>,
}

// TODO: variable entity needs an entity on line 105
pub fn velocity_shrink(
    mut query: Query<(&mut Velocity, &Transform), With<Camera>>,
    rapier_context: Res<RapierContext>,
) {
    let (mut vel, transform) = query.single_mut();

    // Dummy for readability
    let dir = &mut vel;

    // Checking if velocities are exceeding what they need to be
    dir.x.clamp(-CAMERA_SPEED, CAMERA_SPEED);
    dir.z.clamp(-CAMERA_SPEED, CAMERA_SPEED);
    
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