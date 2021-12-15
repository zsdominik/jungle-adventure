mod components;
mod systems;

use bevy::{
    prelude::{App, IntoSystem},
    DefaultPlugins,
};
use bevy_console::{ConsoleConfiguration, ConsolePlugin};
use systems::{
    camera_movement_system::move_camera,
    console_system::listen_to_console_events,
    game_initializer_system::{init_level, setup_player_sprite, spawn_camera},
    keyboard_input_system::player_control,
};
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .add_startup_system(spawn_camera.system())
        .add_startup_system(setup_player_sprite.system())
        .add_startup_system(init_level.system())
        .add_system(listen_to_console_events.system())
        .add_system(player_control.system())
        .add_system(move_camera.system())
        .run();
}
