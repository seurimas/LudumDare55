use bevy::log::tracing_subscriber::fmt::time;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Resource, Asset, TypePath, Clone)]
pub struct Story {
    pub waves: Vec<String>,
}

pub fn start_story(
    mut story: ResMut<Story>,
    summon_assets: Res<SummonsAssets>,
    story_asset: Res<Assets<Story>>,
) {
    *story = story_asset
        .get(summon_assets.story_teller.clone())
        .unwrap()
        .clone();
}

pub fn queue_next_wave(
    mut commands: Commands,
    mut story: ResMut<Story>,
    mut next_state: ResMut<NextState<GameState>>,
    mut enemy_minions: ResMut<EnemyMinions>,
    keys: Res<ButtonInput<KeyCode>>,
    summon_assets: Res<SummonsAssets>,
    wave_assets: Res<Assets<SummonedMinions>>,
    summon_entities: Query<Entity, With<Summon>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        let wave = summon_assets.waves.get(&*story.waves.remove(0)).unwrap();
        let wave = wave_assets.get(wave).unwrap();
        enemy_minions.0 = wave.clone();
        next_state.set(GameState::Battling);
        for entity in summon_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Resource, Default)]
pub struct SpawnProgress(pub f32);

pub fn spawn_all_summons(
    mut progress: ResMut<SpawnProgress>,
    time: Res<Time>,
    mut commands: Commands,
    mut my_minions: ResMut<SummonedMinions>,
    mut enemy_minions: ResMut<EnemyMinions>,
    known_summons: Res<KnownSummons>,
    summons: Res<SummonsAssets>,
    textures: Res<TextureAssets>,
    brains: Res<BrainAssets>,
    brain_assets: Res<Assets<CharacterBrainDef>>,
    summon_assets: Res<Assets<SummonType>>,
) {
    progress.0 += time.delta_seconds();
    if progress.0 < 0.5 {
        return;
    }
    progress.0 = 0.;
    let (faction, (x, y, summon)) = if my_minions.summons() > 0 {
        (Faction::Player, my_minions.pop_summon().unwrap())
    } else if enemy_minions.0.summons() > 0 {
        (Faction::Enemy, enemy_minions.0.pop_summon().unwrap())
    } else {
        // Prepare for next time.
        progress.0 = 0.;
        return;
    };
    let summon_type = if faction == Faction::Player {
        known_summons.get(&summon)
    } else {
        let summon_handle = summons.npc_summons.get(&*summon).unwrap();
        summon_assets.get(summon_handle).unwrap().clone()
    };
    let summoned = spawn_summon(&mut commands, &textures, summon_type.clone(), x, y, true);
    let brain = summon_type.get_brain(&brains).unwrap();
    let brain_def = brain_assets.get(brain).unwrap();
    commands.entity(summoned).insert((
        Into::<CharacterStats>::into(summon_type),
        CharacterBrain::new(brain_def),
        faction,
    ));
}
