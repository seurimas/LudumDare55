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
            .add_systems(
                OnEnter(GameState::Summoning),
                (spawn_narration_box, show_next_wave),
            )
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

fn show_next_wave(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    story: Res<Story>,
    waves: Res<Assets<SummonedMinions>>,
    summon_types: Res<Assets<SummonType>>,
    summons: Res<SummonsAssets>,
) {
    if let Some(wave) = story.waves.get(0) {
        let minions = summons.waves.get(&*wave.to_string()).unwrap();
        let wave = waves.get(minions).unwrap();
        for ((x, y), summon) in wave.iter() {
            let summon_type = summons
                .npc_summons
                .get(&*summon.to_string())
                .or_else(|| summons.player_summons.get(&*summon.to_string()))
                .and_then(|handle| summon_types.get(handle))
                .unwrap()
                .clone();
            spawn_summon(&mut commands, &textures, summon_type, *x, *y, false);
        }
    }
}
