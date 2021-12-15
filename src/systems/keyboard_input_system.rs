use bevy::prelude::*;

use crate::components::player::Player;
use crate::systems::sprite_animation_system::animate_player_sprite;

pub fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_sprite_transform_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut sprite_animation_query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    let mut player_transform = player_sprite_transform_query
        .single_mut()
        .expect("only one player sprite expected!");

    if keyboard_input.pressed(KeyCode::D) {
        player_transform.rotation = Quat::default();
        player_transform.translation.x += 5.0;
        animate_player_sprite(&mut sprite_animation_query, &time, &texture_atlases);
    }

    if keyboard_input.pressed(KeyCode::A) {
        player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        player_transform.translation.x -= 5.0;
        animate_player_sprite(&mut sprite_animation_query, &time, &texture_atlases);
    }
}
