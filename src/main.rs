use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .run();
}