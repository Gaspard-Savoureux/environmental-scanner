use macroquad::prelude::*;

/// Apply the input given by the user.
///
/// Take the current pitch and yaw then returns the updated values.
pub fn apply_input(moving_step: &f32, pitch: &mut f32, yaw: &mut f32) {
    if is_key_down(KeyCode::Up) {
        *pitch += moving_step;
    }

    if is_key_down(KeyCode::Down) {
        *pitch -= moving_step;
    }

    if is_key_down(KeyCode::Left) {
        *yaw -= moving_step;
    }

    if is_key_down(KeyCode::Right) {
        *yaw += moving_step;
    }
}
