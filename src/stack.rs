use bevy::prelude::*;

use crate::*;
use block::*;
use piece::*;

#[derive(Component)]
pub struct Stack;

pub fn check_piece_collision(
    mut commands: Commands,
    query_falling: Query<(Entity, &Block), With<Falling>>,
    query_stack: Query<(Entity, &Block), With<Stack>>,
) {
    let mut stop_falling = false;
    'outer: for (_, falling_block) in query_falling.iter() {
        // Check collision with the playfield bottom
        if falling_block.position.0 == 1 {
            stop_falling = true;
            break 'outer;
        }
        // Check collision with the stack
        for (_, stack_block) in query_stack.iter() {
            if falling_block.position.1 == stack_block.position.1
                && falling_block.position.0 == stack_block.position.0 + 1
            {
                stop_falling = true;
                break 'outer;
            }
        }
    }
    if stop_falling {
        query_falling.for_each(|(entity, _)| {
            commands.entity(entity).remove::<Falling>();
            commands.entity(entity).insert(Stack);
        });
    }
}
