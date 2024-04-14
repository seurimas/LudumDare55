// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use LudumDare55::GamePlugin;

mod battle;
mod board;
mod bt;
mod flow;
mod loading;
mod menu;
mod persistence;
mod prelude;
mod state;
mod summoner;
mod summons;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ludum Dare 55".to_string(),
                canvas: Some("#bevy".to_owned()),
                prevent_default_event_handling: false,
                resizable: false,
                resolution: WindowResolution::new(WINDOW_SIZE.0, WINDOW_SIZE.1),
                ..default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
