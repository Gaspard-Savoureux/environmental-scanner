use macroquad::prelude::*;

const MOVING_STEP: f32 = 0.1;
const TARGET: Vec3 = vec3(0., 0., 0.);

#[macroquad::main("scanner-ui")]
async fn main() {
    // Changing the following changes the zoom
    let radius = 40.0;

    // Horizontal and vertical angles (in radians):
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = -10.0;

    loop {
        clear_background(LIGHTGRAY);

        let x = TARGET.x + radius * yaw.cos() * pitch.cos();
        let y = TARGET.y + radius * pitch.sin();
        let z = TARGET.z + radius * yaw.sin() * pitch.cos();

        let camera_position = vec3(x, y, z);

        // Going 3d!
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

        if is_key_down(KeyCode::Up) {
            pitch += MOVING_STEP;
        }
        if is_key_down(KeyCode::Down) {
            pitch -= MOVING_STEP;
        }
        if is_key_down(KeyCode::Left) {
            yaw -= MOVING_STEP;
        }
        if is_key_down(KeyCode::Right) {
            yaw += MOVING_STEP;
        }

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        draw_cube(vec3(0., 0., 0.), vec3(0.2, 0.8, 0.2), None, BLACK);
        draw_sphere(vec3(0., 1., 1.), 0.1, None, GREEN);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        // Back to screen space, render some text
        set_default_camera();
        draw_text("3D scanner", 10.0, 20.0, 30.0, BLACK);
        draw_text(
            format!("x: {}, y: {}, z: {}", x, y, z).as_str(),
            10.0,
            40.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!("pitch: {}, yaw: {}", pitch, yaw).as_str(),
            10.0,
            60.0,
            20.0,
            BLACK,
        );

        next_frame().await
    }
}
