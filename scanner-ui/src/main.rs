use key_mapping::apply_input;
use macroquad::{prelude::*, ui::root_ui};
use ui::{show_debug_info, show_settings, Settings};
mod key_mapping;
mod ui;

const MOVE_SPEED: f32 = 0.1;
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
        radius: 40.,
        yaw: 0.,
        pitch: -10., // set to -10 to rise camera on start
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

        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings") {
            settings.toggle_display();
        }

        if settings.debug {
            show_debug_info((x, y, z), (ctx.pitch, ctx.yaw), &settings, text_color);
        }

        if settings.display {
            show_settings(&mut settings);
        }

        next_frame().await
    }
}
