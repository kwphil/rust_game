use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod platform;
mod character;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(platform::PlatformPlugin)
        .run();
}
