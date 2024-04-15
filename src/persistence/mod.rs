pub mod runes;
pub use runes::*;
pub mod ui;
pub use ui::*;

use crate::prelude::*;

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveData>();

        app.add_systems(Update, (save_on_click).run_if(in_state(GameState::Victory)));

        app.add_systems(Update, load_on_click.run_if(in_state(GameState::Menu)));

        #[cfg(target_arch = "wasm32")]
        app.add_systems(OnExit(GameState::Loading), wait_for_loads);

        #[cfg(target_arch = "wasm32")]
        app.add_systems(Update, load_on_event.run_if(in_state(GameState::Menu)));

        #[cfg(target_arch = "wasm32")]
        app.add_systems(OnExit(GameState::Victory), hide_clipboard);

        #[cfg(target_arch = "wasm32")]
        app.add_systems(OnExit(GameState::Menu), hide_clipboard);
    }
}
