use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Handle, SpriteSheetBundle, Transform},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use super::sprites::SheetSprites;

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 4.0;
#[allow(dead_code)]
pub struct Tile {
    x: i32,
    y: i32,
    z: i32,
    min: Vec2,
    max: Vec2,
    scaled_position: Vec3,
    sprite_id: u32,
}

impl Tile {
    pub fn new(x: i32, y: i32, z: i32, sprite: SheetSprites) -> Self {
        let scaled_x = x as f32 * (TILE_SIZE * TILE_SCALE);
        let scaled_y = -y as f32 * (TILE_SIZE * TILE_SCALE);
        Tile {
            min: Vec2::new(scaled_x, scaled_y),
            max: Vec2::new(scaled_x + x as f32, scaled_y + y as f32),
            x,
            y: -y,
            z,
            scaled_position: Vec3::new(scaled_x + x as f32, scaled_y + y as f32, z as f32),
            sprite_id: sprite as u32,
        }
    }

    pub fn spawn_tile(&self, commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
        commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: self.scaled_position,
                scale: Vec3::splat(TILE_SCALE),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(self.sprite_id),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        });
    }
}
