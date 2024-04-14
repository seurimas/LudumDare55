use bevy::log::tracing_subscriber::fmt::time;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Resource, Asset, TypePath, Clone)]
pub struct Story {
    pub waves: Vec<String>,
    pub winning_beats: Vec<Vec<StoryBeatType>>,
    pub losing_beats: Vec<Vec<StoryBeatType>>,
    pub agnostic_beats: Vec<Vec<StoryBeatType>>,
}

impl Story {
    pub fn win(&mut self) -> Vec<StoryBeatType> {
        let mut winning = if self.winning_beats.is_empty() {
            vec![]
        } else {
            self.winning_beats.remove(0)
        };
        if !self.agnostic_beats.is_empty() {
            winning.extend(self.agnostic_beats.remove(0));
        }
        winning
    }

    pub fn lose(&mut self) -> Vec<StoryBeatType> {
        let mut losing = if self.losing_beats.is_empty() {
            vec![]
        } else {
            self.losing_beats.remove(0)
        };
        if !self.agnostic_beats.is_empty() {
            losing.extend(self.agnostic_beats.remove(0));
        }
        losing
    }
}

#[derive(Resource)]
pub struct StoryBeat {
    pub mana_gained: i32,
    pub narration: Vec<String>,
}

impl Default for StoryBeat {
    fn default() -> Self {
        Self {
            mana_gained: 0,
            narration: vec![
                "Welcome to Summoner's Chess!".to_string(),
                "You've already selected your first summon,\nwhich you can see to the right"
                    .to_string(),
                "You can click on the summon or press\nthe number key to select it".to_string(),
                "Then click on the board or press the\nnumber key to place it".to_string(),
                "When you have finished placing your\nsummons, press Enter to start the battle"
                    .to_string(),
                "When the battle starts, enemies will\nspawn on the opposite side of the board"
                    .to_string(),
                "Your creatures have it from there,\nSummoner!".to_string(),
            ],
        }
    }
}

impl StoryBeat {
    pub fn reset(&mut self) {
        self.mana_gained = 0;
        self.narration.clear();
    }

    pub fn apply(&mut self, beats: Vec<StoryBeatType>) {
        for beat in beats {
            match beat {
                StoryBeatType::GainMana(mana) => self.mana_gained += mana,
                StoryBeatType::Narration(narration) => self.narration.push(narration),
            }
        }
    }

    pub fn narrating(&self) -> bool {
        !self.narration.is_empty()
    }

    pub fn get_active_narration(&self) -> Option<&String> {
        self.narration.first()
    }

    pub fn advance_narration(&mut self) {
        self.narration.remove(0);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StoryBeatType {
    GainMana(i32),
    Narration(String),
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
    story_beat: Res<StoryBeat>,
    mut next_state: ResMut<NextState<GameState>>,
    mut enemy_minions: ResMut<EnemyMinions>,
    keys: Res<ButtonInput<KeyCode>>,
    summon_assets: Res<SummonsAssets>,
    wave_assets: Res<Assets<SummonedMinions>>,
    summon_entities: Query<Entity, With<Summon>>,
    my_minions: Res<SummonedMinions>,
    sounds: Res<AudioAssets>,
) {
    if keys.just_pressed(KeyCode::Enter) && !story_beat.narrating() {
        if my_minions.summons() == 0 {
            commands.spawn(AudioBundle {
                source: sounds.error.clone(),
                ..Default::default()
            });
            return;
        }
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
    sounds: Res<AudioAssets>,
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
    commands.spawn(AudioBundle {
        source: sounds
            .summon_stings
            .get(summon_type.tribe.sting())
            .unwrap()
            .clone(),
        ..Default::default()
    });
    commands.entity(summoned).insert((
        Into::<CharacterStats>::into(summon_type),
        CharacterBrain::new(brain_def),
        faction,
    ));
}
