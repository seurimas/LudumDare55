use bevy::{asset::AssetPath, log::tracing_subscriber::fmt::time, utils::Uuid};

use crate::{battle::DeathCharacterBrain, persistence::SaveData, prelude::*, summoner::NextWave};

#[derive(Serialize, Deserialize, Default, Resource, Asset, TypePath, Clone)]
pub struct Story {
    pub waves: Vec<String>,
    pub winning_beats: Vec<Vec<StoryBeatType>>,
    pub losing_beats: Vec<Vec<StoryBeatType>>,
    pub agnostic_beats: Vec<Vec<StoryBeatType>>,
}

impl Story {
    pub fn from_save_data(
        save: &SaveData,
        wave_assets: &mut Assets<SummonedMinions>,
        summon_assets: &mut SummonsAssets,
    ) -> Self {
        summon_assets.waves = HashMap::new();
        let mut wave_names = vec![];
        for (i, wave) in save.armies.iter().enumerate() {
            let mirrored_army = wave.mirror();
            let wave_name = format!("wave_{}", i);
            wave_names.push(wave_name.clone());
            let handle = wave_assets.add(mirrored_army);
            summon_assets.waves.insert(
                FileStem::from_asset_path(&AssetPath::parse(&wave_name)),
                handle,
            );
        }
        Self {
            waves: wave_names,
            winning_beats: vec![],
            losing_beats: vec![],
            agnostic_beats: vec![
                vec![StoryBeatType::GainMana(1)], // Before 1
                vec![StoryBeatType::GainMana(1)], // Before 2
                vec![StoryBeatType::GainMana(1)], // Before 3
                vec![StoryBeatType::GainMana(1)], // Before 4
                vec![StoryBeatType::GainMana(1)], // Before Boss.
                vec![StoryBeatType::GainMana(3)], // Before 5
                vec![StoryBeatType::GainMana(1)], // Before 6
                vec![StoryBeatType::GainMana(1)], // Before 7
                vec![StoryBeatType::GainMana(1)], // Before 8
                vec![StoryBeatType::GainMana(2)], // Before Boss
            ],
        }
    }
}

impl Story {
    pub fn win(&mut self) -> Vec<StoryBeatType> {
        if !self.losing_beats.is_empty() {
            self.losing_beats.remove(0);
        }
        let mut winning = if self.winning_beats.is_empty() {
            if self.waves.is_empty() {
                vec![StoryBeatType::GameOver(true)]
            } else {
                vec![]
            }
        } else {
            self.winning_beats.remove(0)
        };
        if !self.agnostic_beats.is_empty() {
            winning.extend(self.agnostic_beats.remove(0));
        }
        winning
    }

    pub fn lose(&mut self) -> Vec<StoryBeatType> {
        if !self.winning_beats.is_empty() {
            self.winning_beats.remove(0);
        }
        let mut losing = if self.losing_beats.is_empty() {
            vec![StoryBeatType::GameOver(false)]
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
    pub victory: bool,
    pub defeat: bool,
}

impl Default for StoryBeat {
    fn default() -> Self {
        Self {
            mana_gained: 0,
            narration: vec![
                "Welcome to Summoner's Chess!\nSave the town and help reclaim the land with your magic!".to_string(),
                "You've already selected your first summon, which you can see to the right"
                    .to_string(),
                "You can click on the summon or press the hotkey to select it".to_string(),
                "Then click on the board or press the hotkey to place it".to_string(),
                "When you have finished placing your summons, press Enter to start the battle"
                    .to_string(),
                "When the battle starts, enemies will spawn on the opposite side of the board"
                    .to_string(),
                "Your creatures have it from there, Summoner! When the battle is done, maybe you'll have learned a few things...".to_string(),
            ],
            victory: false,
            defeat: false,
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
                StoryBeatType::GameOver(victory) => {
                    self.victory = victory;
                    self.defeat = !victory;
                }
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
    GameOver(bool),
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
    mut save: ResMut<SaveData>,
    story_beat: Res<StoryBeat>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_wave: ResMut<NextWave>,
    mut enemy_minions: ResMut<EnemyMinions>,
    keys: Res<ButtonInput<KeyCode>>,
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
        save.armies.push(my_minions.clone());
        let wave = core::mem::take(&mut next_wave.0);
        enemy_minions.0 = wave;
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
        let summon_handle = summons
            .npc_summons
            .get(&*summon)
            .or_else(|| summons.player_summons.get(&*summon))
            .unwrap();
        summon_assets.get(summon_handle).unwrap().clone()
    };
    let summoned = spawn_summon(&mut commands, &textures, summon_type.clone(), x, y, true);

    let brain = summon_type.get_brain(&brains).unwrap();
    let brain_def = brain_assets.get(brain).unwrap();

    let death_brain = summon_type
        .get_death_brain(&brains)
        .unwrap_or(brains.brains.get("death").unwrap().clone());
    let death_brain_def = brain_assets.get(death_brain).unwrap();

    commands.spawn(AudioBundle {
        source: sounds
            .summon_stings
            .get(summon_type.tribe.sting())
            .unwrap()
            .clone(),
        ..Default::default()
    });
    commands.entity(summoned).insert((
        CharacterBrain::new(brain_def),
        DeathCharacterBrain(CharacterBrain::new(death_brain_def)),
        faction,
    ));
}
