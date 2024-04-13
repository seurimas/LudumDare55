use crate::prelude::*;

mod book;
mod mana;
mod placement;
pub use book::*;
pub use mana::*;
pub use placement::*;

pub struct SummonerPlugin;

impl Plugin for SummonerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mana>()
            // Debug summons.
            .add_systems(OnEnter(GameState::Summoning), setup_summons)
            // Place the summons on the board
            .init_resource::<SummonedMinions>()
            .add_systems(
                Update,
                (place_summon, animate_summons, remove_summon)
                    .run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                OnEnter(GameState::Summoning),
                spawn_summoning_scroll.after(setup_summons),
            )
            .add_systems(
                Update,
                (mana_bar_system, mana_tally_system).run_if(in_state(GameState::Summoning)),
            )
            .add_systems(OnExit(GameState::Summoning), despawn_summoning_scroll);
    }
}

pub fn setup_summons(
    mut commands: Commands,
    summon_types: ResMut<Assets<SummonType>>,
    summons_assets: Res<SummonsAssets>,
) {
    commands.insert_resource(KnownSummons::new(vec![
        summon_types
            .get(summons_assets.skeleton.clone())
            .unwrap()
            .clone(),
        summon_types
            .get(summons_assets.angel.clone())
            .unwrap()
            .clone(),
        summon_types
            .get(summons_assets.watcher.clone())
            .unwrap()
            .clone(),
    ]));
}
