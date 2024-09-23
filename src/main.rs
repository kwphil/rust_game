use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod platform;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .run();
}
