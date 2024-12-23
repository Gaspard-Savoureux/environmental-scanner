use key_mapping::apply_input;
use macroquad::{prelude::*, ui::root_ui};
use ui::{show_debug_info, show_settings, Settings};
mod key_mapping;
mod ui;

const MOVING_STEP: f32 = 0.1;
const TARGET: Vec3 = vec3(0., 0., 0.);
const DOT_SIZE: f32 = 0.1;

#[macroquad::main("scanner-ui")]
async fn main() {
    // Changing the following changes the zoom
    let radius = 40.0;

    // Horizontal and vertical angles (in radians):
    let mut yaw: f32 = 0.0;
    // let mut pitch: f32 = 0.0;
    let mut pitch: f32 = -10.0;

    let mut vec = Vec::new();
    for i in -10..11 {
        for j in 0..4 {
            vec.push(vec3(i as f32, j as f32, 10.));
            vec.push(vec3(i as f32, j as f32, -10.));
            vec.push(vec3(10., j as f32, i as f32));
            vec.push(vec3(-10., j as f32, i as f32));
        }
    }

    let mut settings = Settings::builder().build();
    let mut text_color: Color;

    loop {
        if settings.dark_theme {
            clear_background(BLACK);
            text_color = WHITE;
        } else {
            clear_background(LIGHTGRAY);
            text_color = BLACK;
        }
        // clear_background(LIGHTGRAY);

        let x = TARGET.x + radius * yaw.cos() * pitch.cos();
        let y = TARGET.y + radius * pitch.sin();
        let z = TARGET.z + radius * yaw.sin() * pitch.cos();

        let camera_position = vec3(x, y, z);

        set_camera(&Camera3D {
            position: camera_position,
            up: vec3(0., 1., 0.),
            target: TARGET,
            ..Default::default()
        });

        // Closing UI, but will eventually open a menu
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        apply_input(&MOVING_STEP, &mut pitch, &mut yaw);

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

        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings") {
            settings.toggle_display();
        }

        if settings.debug {
            show_debug_info((x, y, z), (pitch, yaw), &settings, text_color);
        }

        if settings.display {
            show_settings(&mut settings);
        }

        next_frame().await
    }
}
