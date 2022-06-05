use bevy::{core::FixedTimestep, prelude::*};

mod piece;
mod playfield;

// General game constants
const MAX_COLUMN: u8 = 10;
const MAX_ROW: u8 = 20;
// Size of a tetromino building block
const BLOCK_SIZE: f32 = 20.0;

const TIMESTEP: f64 = 60.0 / 1000.0;

// Colors
const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
const LBLUE: Color = Color::rgb(105.0 / 255.0, 227.0 / 255.0, 250.0 / 255.0);
const DBLUE: Color = Color::rgb(21.0 / 255.0, 2.0 / 255.0, 245.0 / 255.0);
const ORANGE: Color = Color::rgb(238.0 / 255.0, 123.0 / 255.0, 50.0 / 255.0);
const YELLOW: Color = Color::rgb(250.0 / 255.0, 223.0 / 255.0, 75.0 / 255.0);
const GREEN: Color = Color::rgb(145.0 / 255.0, 250.0 / 255.0, 77.0 / 255.0);
const RED: Color = Color::rgb(233.0 / 255.0, 55.0 / 255.0, 68.0 / 255.0);
const MAGENTA: Color = Color::rgb(165.0 / 255.0, 34.0 / 255.0, 238.0 / 255.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "RuST TeTRiS".to_string(),
            width: 400.,
            height: 500.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_startup_system(playfield::playfield_setup)
        .add_startup_system(piece::new_piece)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(piece::move_piece_down),
        )
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn new_rect(x: f32, y: f32, width: f32, height: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(width, height, 0.0),
            ..default()
        },
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

fn row_to_y(row: u8) -> f32 {
    BLOCK_SIZE * (row as f32 - (MAX_ROW + 1) as f32 / 2.0)
}

fn col_to_x(col: u8) -> f32 {
    BLOCK_SIZE * (col as f32 - (MAX_COLUMN + 1) as f32 / 2.0)
}
