use bevy::{
    core::Timer,
    math::{Vec2, Vec3},
    prelude::{
        AssetServer, Assets, Commands, OrthographicCameraBundle, Res, ResMut, SpriteSheetBundle,
        Transform,
    },
    sprite::TextureAtlas,
};

use crate::components::{player::Player, sprites::SheetSprites, tile::Tile};

pub(crate) fn setup_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites\\gold-guy\\walking.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player {});
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub(crate) fn init_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites\\sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.2, 16.0), 17, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for x in 0..10 {
        let tile = Tile::new(x, 1, 0, SheetSprites::BluePlatform03);
        tile.spawn_tile(&mut commands, texture_atlas_handle.clone());
    }

    let tile = Tile::new(1, 0, 0, SheetSprites::Table02);
    tile.spawn_tile(&mut commands, texture_atlas_handle);
}
