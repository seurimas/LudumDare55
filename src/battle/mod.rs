use crate::{
    prelude::*,
    summons::{spawn_summon, Summon},
};

pub mod bt;
pub mod loot;
pub mod runner;
pub mod stats;
pub struct BattlePlugin;
pub use bt::*;
pub use loot::*;
pub use runner::*;
pub use stats::*;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnOrder>()
            .init_resource::<BattleSpeed>()
            .add_systems(
                Update,
                crate::summoner::animate_summons.run_if(in_state(GameState::Battling)),
            )
            .add_systems(
                Update,
                (
                    run_battle,
                    prune_turn_order,
                    prune_dead_entities,
                    end_battle,
                )
                    .run_if(in_state(GameState::Battling)),
            )
            .add_systems(OnEnter(GameState::Looting), setup_loot_screen)
            .add_systems(
                Update,
                handle_loot_button_click.run_if(in_state(GameState::Looting)),
            )
            .add_systems(OnExit(GameState::Looting), cleanup_loot_screen);
    }
}
