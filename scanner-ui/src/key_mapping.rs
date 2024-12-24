use macroquad::prelude::*;

use crate::{ui::Settings, Context};

/// Apply the input given by the user.
///
/// Take the current pitch and yaw then returns the updated values.
pub fn apply_input(
    ctx: &mut Context,
    // moving_step: &f32,
    // pitch: &mut f32,
    // yaw: &mut f32,
    // radius: &mut f32,
    settings: &mut Settings,
) {
    #[cfg_attr(any(), rustfmt::skip)]
    { // Camera control //
    // Arrow mouvements control
    if is_key_down(KeyCode::Up)    { ctx.pitch += ctx.move_speed;}
    if is_key_down(KeyCode::Down)  { ctx.pitch -= ctx.move_speed; }
    if is_key_down(KeyCode::Left)  { ctx.yaw   -= ctx.move_speed; }
    if is_key_down(KeyCode::Right) { ctx.yaw   += ctx.move_speed; }

    // Mouse mouvements control
    let delta = get_frame_time();
    let mouse_position: Vec2 = mouse_position().into();
    let mouse_delta = mouse_position - ctx.last_mouse_position;
    ctx.last_mouse_position = mouse_position;

    if is_mouse_button_down(MouseButton::Left) { ctx.yaw   += mouse_delta.x * delta * 0.1; }
    if is_mouse_button_down(MouseButton::Left) { ctx.pitch -= mouse_delta.y * delta * 0.1; }
    
    // mouse_wheel zoom
    let (_, mouse_wheel_y) = mouse_wheel();
    if      mouse_wheel_y > 0. { ctx.radius += 1.; } 
    else if mouse_wheel_y < 0. { ctx.radius -= 1.; }
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
