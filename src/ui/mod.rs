use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

pub mod hud;
pub mod settings;

pub fn set_primary_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bullet Heaven".into(),
            resolution: (1920., 1080.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
