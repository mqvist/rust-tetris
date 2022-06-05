use rand::Rng;

use crate::*;

#[derive(Component)]
struct Block;

type BlockPos = (u8, u8);

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
pub struct Piece {
    position: BlockPos,
}

pub fn new_piece(mut commands: Commands, query: Query<With<Piece>>) {
    // Do nothing if we already have a piece
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
                parent
                    .spawn_bundle(new_block(
                        shape.start_pos.0 + *r,
                        shape.start_pos.1 + *c,
                        shape.color,
                    ))
                    .insert(Block);
            }
        })
        .insert(Piece {
            position: shape.start_pos,
        });
}

fn new_block(row: u8, col: u8, color: Color) -> SpriteBundle {
    let x = col_to_x(col);
    let y = row_to_y(row);
    new_rect(x, y, BLOCK_SIZE, BLOCK_SIZE, color)
}

pub fn move_piece_down(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Piece)>,
) {
    if query.is_empty() {
        return;
    }
    let (piece_entity, mut piece_transform, mut piece) = query.single_mut();
    //println!("{:?}", piece_transform);
    if piece.position.0 == 1 {
        commands.entity(piece_entity).despawn();
    } else {
        piece.position.0 -= 1;
        piece_transform.translation.y -= BLOCK_SIZE;
    }
}
