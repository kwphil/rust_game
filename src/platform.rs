use bevy::prelude::*;
pub struct platform;
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
        Platform,
    ));
}
