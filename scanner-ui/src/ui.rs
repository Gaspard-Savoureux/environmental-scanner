use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
    Skin,
};

/// set the default style here
fn default_skin() -> Skin {
    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();
    Skin {
        checkbox_style,
        ..root_ui().default_skin()
    }
}

pub struct Grid {
    pub slices: f32,
    pub spacing: f32,
}

pub struct Settings {
    pub display: bool,
    pub dark_theme: bool,
    pub debug: bool,
    pub skin: Skin,
    pub position: Vec2,
    pub window_size: Vec2,
    pub grid: Grid,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder {
            display: None,
            dark_theme: None,
            debug: None,
            skin: None,
            position: None,
            window_size: None,
            grid: None,
        }
    }

    pub fn refresh_position(&mut self) {
        self.position = vec2(screen_width(), screen_height());
        self.position = self.position / 2. - self.window_size / 2.;
    }

    pub fn toggle_display(&mut self) {
        self.display = !self.display;
    }

    pub fn switch_theme(&mut self) {
        self.dark_theme = !self.dark_theme;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn increase_grid_size(&mut self) {
        self.grid.slices += 1.;
    }

    pub fn decrease_grid_size(&mut self) {
        self.grid.slices -= 1.;
    }
}

pub struct SettingsBuilder {
    display: Option<bool>,
    dark_theme: Option<bool>,
    debug: Option<bool>,
    skin: Option<Skin>,
    position: Option<Vec2>,
    window_size: Option<Vec2>,
    grid: Option<Grid>,
}

#[allow(dead_code)]
impl SettingsBuilder {
    pub fn display(mut self, display: bool) -> Self {
        self.display = Some(display);
        self
    }

    pub fn dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn debug(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn skin(mut self, skin: Skin) -> Self {
        self.skin = Some(skin);
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }

    pub fn window_size(mut self, window_size: Vec2) -> Self {
        self.window_size = Some(window_size);
        self
    }

    pub fn grid(mut self, grid: Grid) -> Self {
        self.grid = Some(grid);
        self
    }

    pub fn build(self) -> Settings {
        let window_size = self.window_size.unwrap_or(vec2(320., 400.));
        let position = self.position.unwrap_or(vec2(
            screen_width() / 2. - window_size.x,
            screen_height() / 2. - window_size.y,
        ));

        Settings {
            display: self.display.unwrap_or(false),
            dark_theme: self.dark_theme.unwrap_or(false),
            debug: self.debug.unwrap_or(false),
            skin: self.skin.unwrap_or(default_skin()),
            position,
            window_size,
            grid: self.grid.unwrap_or(Grid {
                slices: 20.,
                spacing: 1.,
            }),
        }
    }
}

pub fn show_settings(settings: &mut Settings) {
    root_ui().push_skin(&settings.skin);
    settings.refresh_position();

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Settings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.checkbox(hash!(), "Dark theme", &mut settings.dark_theme);
            ui.checkbox(hash!(), "Debug mode", &mut settings.debug);

            // Grid size controlleur
            ui.label(None, "Grid size [20 .. 100]:");
            ui.slider(hash!(), "", 20f32..100f32, &mut settings.grid.slices);

            // Exit button
            if ui.button(
                vec2(settings.window_size.x - 60., settings.window_size.y - 60.),
                "Close",
            ) {
                settings.toggle_display();
                ui.close_current_window();
            }
        });
}

/// Shows debuging info such as camera position, pitch, yaw, current screen size, etc.
pub fn show_debug_info(
    (x, y, z): (f32, f32, f32),
    (pitch, yaw): (f32, f32),
    settings: &Settings,
    text_color: Color,
) {
    // Camera position
    draw_text(
        format!("camera position (x: {}, y: {}, z: {})", x, y, z).as_str(),
        10.0,
        40.0,
        20.0,
        text_color,
    );
    // camera pitch and yaw
    draw_text(
        format!("pitch: {}, yaw: {}", pitch, yaw).as_str(),
        10.0,
        60.0,
        20.0,
        text_color,
    );

    // current screen size
    draw_text(
        format!(
            "screen_width: {}, screen_height: {}",
            screen_width(),
            screen_height()
        )
        .as_str(),
        10.0,
        80.0,
        20.0,
        text_color,
    );

    // Displaying the settings
    draw_text(
        format!("Displaying settings: {}", settings.display).as_str(),
        10.0,
        100.0,
        20.0,
        text_color,
    );
}
