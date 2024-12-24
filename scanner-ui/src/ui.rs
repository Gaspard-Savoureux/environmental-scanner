use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
    Skin,
};

use crate::key_mapping::KEY_MAPPINGS;

/// set the default style here
pub async fn default_skin() -> Skin {
    // To load font
    // let font = load_ttf_font("resources/fonts/Roboto/Roboto-Medium.ttf").await.unwrap();

    let label_style = root_ui()
        .style_builder()
        // .with_font(&font).unwrap()
        .text_color(BLACK)
        .font_size(16)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font_size(16)
        .text_color(LIGHTGRAY)
        .color(DARKGRAY)
        .build();

    Skin {
        label_style,
        checkbox_style,
        button_style,
        ..root_ui().default_skin()
    }
}

pub struct Grid {
    pub slices: f32,
    pub spacing: f32,
}

pub struct Settings {
    pub display_settings: bool,
    pub display_keymapping: bool,
    pub dark_theme: bool,
    pub debug: bool,
    pub skin: HashMap<String, Skin>,
    pub position: Vec2,
    pub window_size: Vec2,
    pub grid: Grid,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder {
            display_settings: None,
            display_keymapping: None,
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

    pub fn toggle_display_settings(&mut self) {
        self.display_settings = !self.display_settings;
    }

    pub fn toggle_display_keymapping(&mut self) {
        self.display_keymapping = !self.display_keymapping;
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
    display_settings: Option<bool>,
    display_keymapping: Option<bool>,
    dark_theme: Option<bool>,
    debug: Option<bool>,
    skin: Option<HashMap<String, Skin>>,
    position: Option<Vec2>,
    window_size: Option<Vec2>,
    grid: Option<Grid>,
}

#[allow(dead_code)]
impl SettingsBuilder {
    pub fn display_settings(mut self, display: bool) -> Self {
        self.display_settings = Some(display);
        self
    }

    pub fn display_keymapping(mut self, display: bool) -> Self {
        self.display_keymapping = Some(display);
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

    pub fn skin(mut self, skin: HashMap<String, Skin>) -> Self {
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

    pub async fn build(self) -> Settings {
        let window_size = self.window_size.unwrap_or(vec2(320., 400.));
        let position = self.position.unwrap_or(vec2(
            screen_width() / 2. - window_size.x,
            screen_height() / 2. - window_size.y,
        ));

        Settings {
            display_settings: self.display_settings.unwrap_or(false),
            display_keymapping: self.display_keymapping.unwrap_or(false),
            dark_theme: self.dark_theme.unwrap_or(false),
            debug: self.debug.unwrap_or(false),
            skin: self.skin.unwrap_or(HashMap::from([(
                "Default".to_string(),
                default_skin().await,
            )])),
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
    let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
    root_ui().push_skin(skin);
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
                settings.toggle_display_settings();
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
        format!("Displaying settings: {}", settings.display_settings).as_str(),
        10.0,
        100.0,
        20.0,
        text_color,
    );

    // Displaying the keymapping
    draw_text(
        format!("Displaying keymapping: {}", settings.display_keymapping).as_str(),
        10.0,
        120.0,
        20.0,
        text_color,
    );
}

pub async fn keymappings_skin() -> Skin {
    // let font = load_ttf_font("resources/fonts/Roboto/Roboto-Regular.ttf").await.unwrap();

    let label_style = root_ui()
        .style_builder()
        // .with_font(&font).unwrap()
        .text_color(BLACK)
        .font_size(16)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let group_style = root_ui()
        .style_builder()
        .color_hovered(RED)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    Skin {
        label_style,
        checkbox_style,
        group_style,
        ..root_ui().default_skin()
    }
}

pub fn show_keymapping(settings: &mut Settings) {
    settings.refresh_position();
    let (_, skin) = settings
        .skin
        .get_key_value(&"Keymapping".to_string())
        .unwrap();

    let mut close_clicked = false;

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Keymappings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.push_skin(skin);
            for (key, description) in KEY_MAPPINGS {
                ui.separator();
                ui.group(hash!(key, 1), vec2(280., 60.), |inner_ui| {
                    inner_ui.label(None, key);
                    inner_ui.separator();
                    inner_ui.same_line(20.);
                    inner_ui.label(None, description);
                });
                ui.separator();
            }

            // Exit button
            if ui.button(None, "Close") {
                close_clicked = true;
                ui.close_current_window();
            }

            ui.separator();
            ui.pop_skin();
        });

    if close_clicked {
        settings.toggle_display_keymapping();
    }
}
