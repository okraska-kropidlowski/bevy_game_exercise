use bevy::prelude::*;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
    .spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(SnakeHead)
    .insert(Position { x: 3, y: 3 }) // <--
    .insert(Size::square(0.8)); // <--
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<(&SnakeHead, &mut Transform)>,
) {
    for (_head, mut transform) in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y -= 2.;
        }
    }
}

fn main() {
    App::new()
        .add_systems(Update,(setup_camera, spawn_snake, snake_movement))
        .add_plugins(DefaultPlugins)
        .run();
}