use bevy::{
    input::Input,
    prelude::{KeyCode, Query, Res, Transform, With},
    render::camera::Camera,
};

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query
        .single_mut()
        .expect("only one camera expected!");

    if keyboard_input.pressed(KeyCode::D) {
        camera_transform.translation.x += 5.0;
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.translation.x -= 5.0;
    }
}
