use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Startup, spawn_platform);
    }
}

#[derive(Component)]
pub struct Platform;

fn spawn_platform(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::new(800.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0), // Position below the player
            ..Default::default()
        },
        Collider::cuboid(400.0, 25.0, 0.1)
        Platform,
    ));
}
