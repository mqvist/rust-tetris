use crate::*;

const PLAYFIELD_WIDTH: f32 = BLOCK_SIZE * MAX_COLUMN as f32;
const PLAYFIELD_HEIGHT: f32 = BLOCK_SIZE * MAX_ROW as f32;

// Walls
enum WallSide {
    Left,
    Right,
    Top,
    Bottom,
}
pub fn playfield_setup(mut commands: Commands) {
    commands.spawn_bundle(new_wall(WallSide::Left));
    commands.spawn_bundle(new_wall(WallSide::Right));
    commands.spawn_bundle(new_wall(WallSide::Top));
    commands.spawn_bundle(new_wall(WallSide::Bottom));
}

fn new_wall(side: WallSide) -> SpriteBundle {
    let left_edge = -PLAYFIELD_WIDTH / 2.0;
    let top_edge = PLAYFIELD_HEIGHT / 2.0;
    let right_edge = PLAYFIELD_WIDTH / 2.0;
    let bottom_edge = -PLAYFIELD_HEIGHT / 2.0;
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
