use macroquad::prelude::*;

use crate::{ui::Settings, Context, MOVE_SPEED, PITCH, RADIUS, YAW};

pub const KEY_MAPPINGS: [(&str, &str); 10] = [
    ("[arrows]", "Control the camera with the arrows"),
    ("[LeftClick + mouse mouvement]", "Control the camera"),
    ("[Escape]", "Open/Close settings"),
    ("[Q]", "Quit the application"),
    ("[D]", "Toggle the debug output"),
    ("[T]", "Switch theme"),
    ("[Shift + \"+\"]", "Increase grid size"),
    ("[Shift + \"-\"]", "Decrease grid size"),
    ("[Mousewheel UP]", "Zoom"),
    ("[Mousewheel Down]", "Unzoom"),
];

/// Apply the input given by the user.
///
/// Take the current pitch and yaw then returns the updated values.
pub fn apply_input(ctx: &mut Context, settings: &mut Settings) {
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

    if is_mouse_button_down(MouseButton::Left) { ctx.yaw   += mouse_delta.x * delta * MOVE_SPEED; }
    if is_mouse_button_down(MouseButton::Left) { ctx.pitch -= mouse_delta.y * delta * MOVE_SPEED; }
    
    // mouse_wheel zoom
    let (_, mouse_wheel_y) = mouse_wheel();
    if      mouse_wheel_y > 0. { ctx.radius -= 1.; } 
    else if mouse_wheel_y < 0. { ctx.radius += 1.; }

    // reset camera
    if is_key_pressed(KeyCode::R) {
        ctx.radius = RADIUS;
        ctx.yaw = YAW;
        ctx.pitch = PITCH;
    }
    }

    #[cfg_attr(any(), rustfmt::skip)]
    { // Settings related //
    if is_key_pressed(KeyCode::Escape) { settings.toggle_display_settings(); }
    if is_key_pressed(KeyCode::K)      { settings.toggle_display_keymapping(); }
    if is_key_pressed(KeyCode::D)      { settings.toggle_debug(); }
    if is_key_pressed(KeyCode::T)      { settings.switch_theme(); }
    
    // Grid //
    if (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
    && is_key_down(KeyCode::Equal)     { settings.increase_grid_size(); }
    if (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
    && is_key_down(KeyCode::Minus)     { settings.decrease_grid_size(); }
    }
}
