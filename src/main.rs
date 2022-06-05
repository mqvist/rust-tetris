use bevy::prelude::*;

// General game constants
const MAX_COLUMN: u8 = 10;
const MAX_ROW: u8 = 20;

// Colors
const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
const LBLUE: Color = Color::rgb(105.0 / 255.0, 227.0 / 255.0, 250.0 / 255.0);
const DBLUE: Color = Color::rgb(21.0 / 255.0, 2.0 / 255.0, 245.0 / 255.0);
const ORANGE: Color = Color::rgb(238.0 / 255.0, 123.0 / 255.0, 50.0 / 255.0);
const YELLOW: Color = Color::rgb(250.0 / 255.0, 223.0 / 255.0, 75.0 / 255.0);
const GREEN: Color = Color::rgb(145.0 / 255.0, 250.0 / 255.0, 77.0 / 255.0);
const RED: Color = Color::rgb(233.0 / 255.0, 55.0 / 255.0, 68.0 / 255.0);
const MAGENTA: Color = Color::rgb(165.0 / 255.0, 34.0 / 255.0, 238.0 / 255.0);

// Size of tetromino building blocks
const BLOCK_SIZE: f32 = 20.0;

#[derive(Component)]
struct Block;

type BlockPos = (u8, u8);

// Walls
enum WallSide {
    Left,
    Right,
    Top,
    Bottom,
}

struct Tetromino {
    blocks: [BlockPos; 4],
    start_pos: BlockPos,
    color: Color,
}

const TETROMINOS: [Tetromino; 2] = [
    // I
    Tetromino {
        blocks: [(0, 0), (0, 1), (0, 2), (0, 3)],
        start_pos: (21, 4),
        color: LBLUE,
    },
    // J
    Tetromino {
        blocks: [(1, 0), (0, 0), (0, 1), (0, 2)],
        start_pos: (21, 4),
        color: DBLUE,
    },
];

#[derive(Component)]
struct Piece;

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
        .add_startup_system(gamearea_setup)
        .add_startup_system(new_piece)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn gamearea_setup(mut commands: Commands) {
    commands.spawn_bundle(new_wall(WallSide::Left));
    commands.spawn_bundle(new_wall(WallSide::Right));
    commands.spawn_bundle(new_wall(WallSide::Top));
    commands.spawn_bundle(new_wall(WallSide::Bottom));
}

fn new_wall(side: WallSide) -> SpriteBundle {
    let left_edge = col_to_x(1) - BLOCK_SIZE / 2.0;
    let top_edge = row_to_y(MAX_ROW + 1) - BLOCK_SIZE / 2.0;
    let right_edge = col_to_x(MAX_COLUMN) + BLOCK_SIZE / 2.0;
    let bottom_edge = row_to_y(0) + BLOCK_SIZE / 2.0;
    let playfield_height = MAX_ROW as f32 * BLOCK_SIZE;
    let playfield_width = MAX_COLUMN as f32 * BLOCK_SIZE;

    let (x, y, width, height) = match side {
        WallSide::Left => (left_edge, 0.0, 1.0, playfield_height),
        WallSide::Right => (right_edge, 0.0, 1.0, playfield_height),
        WallSide::Top => (0.0, top_edge, playfield_width, 1.0),
        WallSide::Bottom => (0.0, bottom_edge, playfield_width, 1.0),
    };

    new_rect(x, y, width, height, WHITE)
}

fn new_piece(mut commands: Commands) {
    let shape = &TETROMINOS[1];

    commands
        .spawn_bundle(TransformBundle { ..default() })
        .with_children(|parent| {
            for (r, c) in shape.blocks.iter() {
                parent
                    .spawn_bundle(new_block(
                        shape.start_pos.0 + *r,
                        shape.start_pos.1 + *c,
                        shape.color,
                    ))
                    .insert(Block);
            }
        })
        .insert(Piece);
}

fn new_block(row: u8, col: u8, color: Color) -> SpriteBundle {
    let x = col_to_x(col);
    let y = row_to_y(row);
    new_rect(x, y, BLOCK_SIZE, BLOCK_SIZE, color)
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
