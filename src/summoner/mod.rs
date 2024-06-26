use crate::prelude::*;

mod book;
mod help;
mod mana;
mod placement;
pub use book::*;
pub use help::*;
pub use mana::*;
pub use placement::*;

pub struct SummonerPlugin;

impl Plugin for SummonerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mana>()
            .init_resource::<KnownSummons>()
            // Place the summons on the board
            .init_resource::<SummonedMinions>()
            .init_resource::<EnemyMinions>()
            .add_systems(OnEnter(GameState::Summoning), spawn_help_overlay)
            .add_systems(Update, show_hovered_stats)
            .add_systems(OnExit(GameState::Battling), despawn_help_overlay)
            .add_systems(
                Update,
                (place_summon, animate_summons, remove_summon)
                    .run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                Update,
                (
                    mana_bar_system,
                    mana_tally_system,
                    animate_summoning_scroll_opening,
                    show_hotkeys,
                    handle_summon_button_interactions.after(place_summon),
                )
                    .run_if(in_state(GameState::Summoning)),
            )
            .add_systems(
                Update,
                (animate_summoning_scroll_closing,).run_if(in_state(GameState::Battling)),
            )
            .add_systems(
                OnEnter(GameState::Summoning),
                (spawn_summoning_scroll, clear_summons),
            );
    }
}

fn clear_summons(mut commands: Commands, query: Query<(Entity, &Summon)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
