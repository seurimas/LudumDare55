use crate::prelude::*;

pub struct StoryPlugin;

mod story;
pub use story::*;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Story>()
            .init_resource::<SpawnProgress>()
            .add_systems(OnExit(GameState::Loading), start_story)
            .add_systems(
                Update,
                queue_next_wave.run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                Update,
                spawn_all_summons.run_if(in_state(GameState::Battling)),
            );
    }
}
