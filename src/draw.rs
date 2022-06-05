use bevy::prelude::*;

pub fn new_rect(x: f32, y: f32, width: f32, height: f32, color: Color) -> SpriteBundle {
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
