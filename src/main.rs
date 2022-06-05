use bevy::{math::const_vec3, prelude::*};

// General game constants
const MAX_COLUMN: u8 = 10;
const MAX_ROW: u8 = 20;

// Colors
const CYAN: Color = Color::rgb(105.0 / 255.0, 227.0 / 255.0, 250.0 / 255.0);
const BLUE: Color = Color::rgb(21.0 / 255.0, 2.0 / 255.0, 245.0 / 255.0);
const ORANGE: Color = Color::rgb(238.0 / 255.0, 123.0 / 255.0, 50.0 / 255.0);
const YELLOW: Color = Color::rgb(250.0 / 255.0, 223.0 / 255.0, 75.0 / 255.0);
const GREEN: Color = Color::rgb(145.0 / 255.0, 250.0 / 255.0, 77.0 / 255.0);
const RED: Color = Color::rgb(233.0 / 255.0, 55.0 / 255.0, 68.0 / 255.0);
const MAGENTA: Color = Color::rgb(165.0 / 255.0, 34.0 / 255.0, 238.0 / 255.0);

// Size of tetromino building blocks
const BLOCK_SIZE: f32 = 20.0;

#[derive(Component)]
struct Block;

enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}

#[derive(Component)]
struct Piece {
    shape: Tetromino,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 400.,
            height: 500.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_startup_system(new_piece)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn new_piece(mut commands: Commands) {
    let shape = Tetromino::I;

    commands
        .spawn_bundle(TransformBundle { ..default() })
        .insert(Piece { shape })
        .with_children(|parent| {
            parent.spawn_bundle(new_block(MAX_ROW, MAX_COLUMN, RED));
            parent.spawn_bundle(new_block(MAX_ROW, 1, GREEN));
            parent.spawn_bundle(new_block(1, MAX_COLUMN, BLUE));
            parent.spawn_bundle(new_block(1, 1, YELLOW));
        });
}

fn new_block(row: u8, col: u8, color: Color) -> SpriteBundle {
    let x = col_to_x(col);
    let y = row_to_y(row);
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0.0),
            ..default()
        },
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

fn row_to_y(row: u8) -> f32 {
    assert!(row > 0 && row <= MAX_ROW);
    BLOCK_SIZE * (row as f32 - MAX_ROW as f32 / 2.0)
}

fn col_to_x(col: u8) -> f32 {
    assert!(col > 0 && col <= MAX_COLUMN);
    BLOCK_SIZE * (col as f32 - MAX_COLUMN as f32 / 2.0)
}
