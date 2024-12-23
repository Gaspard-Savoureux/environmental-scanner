use macroquad::prelude::*;

use crate::ui::Settings;

/// Apply the input given by the user.
///
/// Take the current pitch and yaw then returns the updated values.
pub fn apply_input(moving_step: &f32, pitch: &mut f32, yaw: &mut f32, settings: &mut Settings) {
    #[cfg_attr(any(), rustfmt::skip)]
    { // Camera control //
    if is_key_down(KeyCode::Up)    { *pitch += moving_step;}
    if is_key_down(KeyCode::Down)  { *pitch -= moving_step; }
    if is_key_down(KeyCode::Left)  { *yaw   -= moving_step; }
    if is_key_down(KeyCode::Right) { *yaw   += moving_step; }
    }

    #[cfg_attr(any(), rustfmt::skip)]
    { // Settings related //
    if is_key_pressed(KeyCode::Escape) { settings.toggle_display(); }
    if is_key_pressed(KeyCode::D)      { settings.toggle_debug(); }
    if is_key_pressed(KeyCode::T)      { settings.switch_theme(); }
    if is_key_pressed(KeyCode::T)      { settings.switch_theme(); }
    
    // Grid //
    if is_key_down(KeyCode::LeftShift) 
    && is_key_down(KeyCode::Equal)     { settings.increase_grid_size(); }
    if is_key_down(KeyCode::LeftShift) 
    && is_key_down(KeyCode::Minus)     { settings.decrease_grid_size(); }
    }
}
