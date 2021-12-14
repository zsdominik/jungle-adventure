mod components;
mod systems;

use bevy::{
    prelude::{App, IntoSystem},
    DefaultPlugins,
};
use systems::{
    camera_movement_system::move_camera,
    game_initializer_system::{init_level, setup_player_sprite},
    keyboard_input_system::player_control_system,
};
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_player_sprite.system())
        .add_startup_system(init_level.system())
        .add_system(player_control_system.system())
        .add_system(move_camera.system())
        .run();
}
