use bevy::{math::const_vec3, prelude::*};

const BRICK_SIZE: Vec3 = const_vec3!([20.0, 20.0, 0.0]);
const BRICK_COLOR: Color = Color::rgb(1.0, 0.2, 0.2);
// Colors
const CYAN: Color = Color::rgb(105.0 / 255.0, 227.0 / 255.0, 250.0 / 255.0);
const BLUE: Color = Color::rgb(21.0 / 255.0, 2.0 / 255.0, 245.0 / 255.0);
const ORANGE: Color = Color::rgb(238.0 / 255.0, 123.0 / 255.0, 50.0 / 255.0);
const YELLOW: Color = Color::rgb(250.0 / 255.0, 223.0 / 255.0, 75.0 / 255.0);
const GREEN: Color = Color::rgb(145.0 / 255.0, 250.0 / 255.0, 77.0 / 255.0);
const RED: Color = Color::rgb(233.0 / 255.0, 55.0 / 255.0, 68.0 / 255.0);
const MAGENTA: Color = Color::rgb(165.0 / 255.0, 34.0 / 255.0, 238.0 / 255.0);


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
