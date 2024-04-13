use crate::{
    prelude::*,
    summons::{spawn_summon, Summon},
};

pub mod bt;
pub mod runner;
pub mod stats;
pub struct BattlePlugin;
pub use bt::*;
pub use runner::*;
pub use stats::*;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnOrder>()
            .init_resource::<BattleSpeed>()
            .add_systems(
                Update,
                (
                    debug_battle_start_system,
                    run_battle,
                    prune_turn_order,
                    prune_dead_entities,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn debug_battle_start_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut summoned: ResMut<SummonedMinions>,
    known_summons: Res<KnownSummons>,
    summon_entities: Query<Entity, With<Summon>>,
    textures: Res<TextureAssets>,
    brains: Res<BrainAssets>,
    brain_assets: Res<Assets<CharacterBrainDef>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        info!("Battle system");
        for entity in summon_entities.iter() {
            commands.entity(entity).despawn();
        }
        for (x, y, summon) in summoned.drain_summons() {
            info!("Summon at {},{} is {:?}", x, y, summon);
            let summon_type = known_summons.get(&summon);
            let summoned = spawn_summon(&mut commands, &textures, summon_type.clone(), x, y, true);
            let brain = summon_type.get_brain(&brains).unwrap();
            let brain_def = brain_assets.get(brain).unwrap();
            commands.entity(summoned).insert((
                Into::<CharacterStats>::into(summon_type),
                CharacterBrain::new(brain_def),
                Faction::Player,
            ));
        }
        for x in 3..=3 {
            let enemy = spawn_summon(
                &mut commands,
                &textures,
                known_summons.get(&"Skeleton".to_string()),
                x,
                4,
                true,
            );
            let brain = brains.brains.get("fighter").unwrap();
            let brain_def = brain_assets.get(brain).unwrap();
            commands.entity(enemy).insert((
                Into::<CharacterStats>::into(known_summons.get(&"Skeleton".to_string())),
                CharacterBrain::new(brain_def),
                Faction::Enemy,
            ));
        }
    }
}
