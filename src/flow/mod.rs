use crate::prelude::*;

pub struct StoryPlugin;

mod game_over;
mod narration;
mod story;
pub use game_over::*;
pub use narration::*;
pub use story::*;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Story>()
            .init_resource::<StoryBeat>()
            .init_resource::<SpawnProgress>()
            .add_systems(OnExit(GameState::Loading), start_story)
            .add_systems(
                Update,
                (queue_next_wave, update_narration_box, advance_narration)
                    .run_if(in_state(GameState::Summoning)),
            )
            .add_systems(OnExit(GameState::Battling), check_for_game_over)
            .add_systems(
                Update,
                spawn_all_summons.run_if(in_state(GameState::Battling)),
            )
            .add_systems(OnEnter(GameState::Summoning), spawn_narration_box)
            .add_systems(
                Update,
                animate_game_over_defeat.run_if(in_state(GameState::Defeat)),
            )
            .add_systems(
                Update,
                animate_game_over_victory.run_if(in_state(GameState::Victory)),
            );
    }
}
