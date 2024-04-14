pub mod runes;
pub use runes::*;
pub mod ui;
pub use ui::*;

use crate::prelude::*;

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveData>();
        #[cfg(target_arch = "wasm32")]
        app.add_system(show_hide_save.run_if(in_state(GameState::Playing)));

        app.add_systems(Update, (save_on_click).run_if(in_state(GameState::Victory)));

        app.add_systems(Update, load_on_click.run_if(in_state(GameState::Menu)));

        #[cfg(target_arch = "wasm32")]
        app.add_system(wait_for_loads.in_schedule(OnEnter(GameState::MainMenu)));

        #[cfg(target_arch = "wasm32")]
        app.add_system(load_on_event.run_if(in_state(GameState::MainMenu)));
    }
}
