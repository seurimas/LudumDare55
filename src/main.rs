// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use LudumDare55::GamePlugin;

mod battle;
mod board;
mod loading;
mod menu;
mod prelude;
mod state;
mod summons;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ludum Dare 55".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        prevent_default_event_handling: false,
                        resizable: false,
                        resolution: WindowResolution::new(948., 533.),
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}
