use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle { // <-- here
            sprite: Sprite {
                color: (SNAKE_HEAD_COLOR),
                ..default()
            },
            transform: Transform {
                scale: Vec3 {
                    x: (10.0),
                    y: (10.0),
                    z: (10.0),
                },
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn main() {
    App::new()
        .add_systems(Update,(setup_camera, spawn_snake))
        .add_plugins(DefaultPlugins)
        .run();
}