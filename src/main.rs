use bevy::{math::const_vec3, prelude::*};

const BRICK_SIZE: Vec3 = const_vec3!([20.0, 20.0, 0.0]);
const BRICK_COLOR: Color = Color::rgb(1.0, 0.2, 0.2);

#[derive(Component)]
struct Brick;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn().insert(Brick).insert_bundle(SpriteBundle {
        transform: Transform {
            scale: BRICK_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        ..default()
    });
}
