use std::collections::HashMap;

use key_mapping::apply_input;
use macroquad::{prelude::*, ui::root_ui};
use ui::{
    default_skin, keymappings_skin, show_debug_info, show_keymapping, show_settings, Settings,
};
mod key_mapping;
mod ui;

// Default context args
const MOVE_SPEED: f32 = 0.1;
const RADIUS: f32 = 40.;
const YAW: f32 = 0.;
const PITCH: f32 = -10.; // set to -10 to rise camera on start

const TARGET: Vec3 = vec3(0., 0., 0.);
const DOT_SIZE: f32 = 0.1;

struct Context {
    pub move_speed: f32,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub last_mouse_position: Vec2,
}

#[macroquad::main("scanner-ui")]
async fn main() {
    let mut ctx: Context = Context {
        move_speed: MOVE_SPEED,
        radius: RADIUS,
        yaw: YAW,
        pitch: PITCH,
        last_mouse_position: mouse_position().into(),
    };

    let mut vec = Vec::new();
    for i in -10..11 {
        for j in 0..4 {
            vec.push(vec3(i as f32, j as f32, 10.));
            vec.push(vec3(i as f32, j as f32, -10.));
            vec.push(vec3(10., j as f32, i as f32));
            vec.push(vec3(-10., j as f32, i as f32));
        }
    }

    let mut settings = Settings::builder()
        .skin(HashMap::from([
            ("Default".to_string(), default_skin().await),
            ("Keymapping".to_string(), keymappings_skin().await),
        ]))
        .build()
        .await;
    let mut text_color: Color;

    loop {
        if settings.dark_theme {
            clear_background(BLACK);
            text_color = WHITE;
        } else {
            clear_background(LIGHTGRAY);
            text_color = BLACK;
        }

        let x = TARGET.x + ctx.radius * ctx.yaw.cos() * ctx.pitch.cos();
        let y = TARGET.y + ctx.radius * ctx.pitch.sin();
        let z = TARGET.z + ctx.radius * ctx.yaw.sin() * ctx.pitch.cos();

        let camera_position = vec3(x, y, z);

        set_camera(&Camera3D {
            position: camera_position,
            up: vec3(0., 1., 0.),
            target: TARGET,
            ..Default::default()
        });

        // Closing UI, but will eventually open a menu
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        apply_input(&mut ctx, &mut settings);

        draw_grid(20, 1., text_color, GRAY);
        draw_grid(
            settings.grid.slices as u32,
            settings.grid.spacing,
            text_color,
            GRAY,
        );

        // Test
        for pos in &vec {
            draw_cube(*pos, vec3(DOT_SIZE, DOT_SIZE, DOT_SIZE), None, GREEN);
        }

        // Back to screen space, render some text
        set_default_camera();
        draw_text("3D scanner", 10.0, 20.0, 30.0, text_color);

        // Buttons
        let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
        root_ui().push_skin(skin);
        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings  ") {
            settings.toggle_display_settings();
        }

        if root_ui().button(vec2(screen_width() - 80., 40.), "Keymapping") {
            settings.toggle_display_keymapping();
        }

        root_ui().pop_skin();

        if settings.display_settings {
            show_settings(&mut settings);
        }

        if settings.display_keymapping {
            show_keymapping(&mut settings);
        }

        if settings.debug {
            show_debug_info((x, y, z), (ctx.pitch, ctx.yaw), &settings, text_color);
        }

        next_frame().await
    }
}
