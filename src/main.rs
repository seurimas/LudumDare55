// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod loading;
mod prelude;
mod state;

use bevy::{window::WindowPlugin, DefaultPlugins};

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ludum Dare 55".to_string(),
                canvas: Some("#bevy".to_owned()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..Default::default()
        }))
        .run();
}
