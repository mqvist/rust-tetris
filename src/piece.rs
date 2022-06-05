use rand::Rng;

use crate::*;
use block::*;

struct Tetromino {
    blocks: [BlockPos; 4],
    start_pos: BlockPos,
    color: Color,
}

const TETROMINOS: [Tetromino; 7] = [
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
    // L
    Tetromino {
        blocks: [(0, 0), (0, 1), (0, 2), (1, 2)],
        start_pos: (21, 4),
        color: ORANGE,
    },
    // O
    Tetromino {
        blocks: [(0, 0), (1, 0), (0, 1), (1, 1)],
        start_pos: (21, 5),
        color: YELLOW,
    },
    // S
    Tetromino {
        blocks: [(0, 0), (0, 1), (1, 1), (1, 2)],
        start_pos: (21, 4),
        color: GREEN,
    },
    // Z
    Tetromino {
        blocks: [(1, 0), (1, 1), (0, 1), (0, 2)],
        start_pos: (21, 4),
        color: RED,
    },
    // T
    Tetromino {
        blocks: [(0, 0), (0, 1), (1, 1), (0, 2)],
        start_pos: (21, 4),
        color: MAGENTA,
    },
];

#[derive(Component)]
pub struct Falling;

pub fn new_piece(mut commands: Commands, query: Query<With<Falling>>) {
    // Do nothing if we already have falling blocks
    if !query.is_empty() {
        return;
    }

    // Pick a random tetromino to spawn
    let mut rng = rand::thread_rng();
    let shape = &TETROMINOS[rng.gen_range(0..TETROMINOS.len())];

    commands
        .spawn_bundle(TransformBundle { ..default() })
        .with_children(|parent| {
            for (r, c) in shape.blocks.iter() {
                let block_row = shape.start_pos.0 + *r;
                let block_col = shape.start_pos.1 + *c;
                parent
                    .spawn_bundle(new_block(block_row, block_col, shape.color))
                    .insert(Block {
                        position: (block_row, block_col),
                    })
                    .insert(Falling);
            }
        });
}

pub fn move_piece_down(mut query: Query<(&mut Transform, &mut Block), With<Falling>>) {
    for (mut transform, mut block) in query.iter_mut() {
        block.position.0 -= 1;
        transform.translation.y -= BLOCK_SIZE;
    }
}
