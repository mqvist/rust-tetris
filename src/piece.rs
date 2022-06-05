use crate::*;

#[derive(Component)]
struct Block;

type BlockPos = (u8, u8);

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

pub fn new_piece(mut commands: Commands) {
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
