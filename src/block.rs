use bevy::prelude::*;

use crate::*;
use draw::*;

#[derive(Component, Debug)]
pub struct Block {
    pub position: BlockPos,
}

pub type BlockPos = (u8, u8);

pub fn new_block(row: u8, col: u8, color: Color) -> SpriteBundle {
    let x = col_to_x(col);
    let y = row_to_y(row);
    new_rect(x, y, BLOCK_SIZE, BLOCK_SIZE, color)
}

pub fn row_to_y(row: u8) -> f32 {
    BLOCK_SIZE * (row as f32 - (MAX_ROW + 1) as f32 / 2.0)
}

pub fn col_to_x(col: u8) -> f32 {
    BLOCK_SIZE * (col as f32 - (MAX_COLUMN + 1) as f32 / 2.0)
}
