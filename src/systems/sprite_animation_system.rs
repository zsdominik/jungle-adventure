use bevy::{
    core::{Time, Timer},
    prelude::{Assets, Handle, Query, Res},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

pub fn animate_player_sprite(
    query: &mut Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    time: &Res<Time>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
