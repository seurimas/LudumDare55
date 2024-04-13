use bevy::utils::HashMap;

use crate::prelude::*;

pub struct SummonsPlugin;

impl Plugin for SummonsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SummonedMinions>()
            .add_systems(OnEnter(GameState::Playing), setup_summons)
            .add_systems(
                Update,
                (place_summon, animate_summons, remove_summon).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Asset, TypePath)]
pub struct SummonType {
    summon_name: String,
    sprite_idx: usize,
    mana_cost: i32,
    health: i32,
    stamina: i32,
    stamina_regen: i32,
    attacks: Vec<Attack>,
    movements: Vec<Movement>,
    brain: String,
}

impl SummonType {
    pub fn name(&self) -> &str {
        &self.summon_name
    }

    pub fn get_brain(&self, brain_assets: &BrainAssets) -> Option<Handle<CharacterBrainDef>> {
        brain_assets.brains.get(&*self.brain.as_str()).cloned()
    }

    pub fn sprite_idx(&self) -> usize {
        self.sprite_idx
    }
}

impl Into<CharacterStats> for SummonType {
    fn into(self) -> CharacterStats {
        CharacterStats {
            health: self.health,
            stamina: self.stamina,
            stamina_regen: self.stamina_regen,
            attacks: self.attacks,
            movements: self.movements,
        }
    }
}

#[derive(Resource, Default)]
pub struct KnownSummons {
    summons: HashMap<String, SummonType>,
}

impl KnownSummons {
    pub fn get(&self, name: &String) -> SummonType {
        self.summons[name].clone()
    }

    pub fn length(&self) -> usize {
        self.summons.len()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Resource, Default)]
pub struct SummonedMinions {
    spawn_locations: HashMap<(usize, usize), String>,
    mana: i32,
    #[serde(skip)]
    mana_locations: HashMap<(usize, usize), i32>,
}

impl SummonedMinions {
    pub fn has_spawn_location(&self, x: usize, y: usize) -> bool {
        self.spawn_locations.contains_key(&(x, y))
    }

    pub fn add_summon(&mut self, summon: SummonType, x: usize, y: usize) {
        self.mana += summon.mana_cost;
        info!("Mana: {}", self.mana);
        self.spawn_locations.insert((x, y), summon.summon_name);
        self.mana_locations.insert((x, y), summon.mana_cost);
    }

    pub fn remove_summon(&mut self, x: usize, y: usize) -> bool {
        if let (Some(summon_name), Some(mana_cost)) = (
            self.spawn_locations.remove(&(x, y)),
            self.mana_locations.remove(&(x, y)),
        ) {
            self.mana -= mana_cost;
            info!("Mana: {}", self.mana);
            true
        } else {
            false
        }
    }

    pub fn drain_summons(&mut self) -> Vec<(usize, usize, String)> {
        let mut drained = Vec::new();
        for ((x, y), summon_name) in self.spawn_locations.iter() {
            drained.push((*x, *y, summon_name.clone()));
        }
        self.spawn_locations.clear();
        self.mana_locations.clear();
        drained
    }
}

#[derive(Component)]
pub struct Summon {
    pub summon_type: SummonType,
    pub time: f32,
    pub x: usize,
    pub y: usize,
}

fn animate_summons(time: Res<Time>, mut query: Query<(&mut Summon, &mut Transform)>) {
    for (mut summon, mut transform) in query.iter_mut() {
        summon.time += time.delta_seconds();
        let offset = summon.time.sin() + PI;
        let translation =
            tile_position_to_translation(summon.x as i32, summon.y as i32) + Vec2::new(0., offset);
        transform.translation = translation.extend(1.);
    }
}

pub fn setup_summons(
    mut commands: Commands,
    summon_types: ResMut<Assets<SummonType>>,
    summons_assets: Res<SummonsAssets>,
) {
    commands.insert_resource(KnownSummons {
        summons: summon_types
            .iter()
            .map(|(handle, summon)| (summon.summon_name.clone(), summon.clone()))
            .collect(),
    });
}

pub fn remove_summon(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    board_mouse_state: Res<BoardMouseState>,
    mut summoned_minions: ResMut<SummonedMinions>,
    summoned_entities: Query<(Entity, &Summon)>,
) {
    if let Some((x, y)) = board_mouse_state.hovered_tile {
        if keyboard_input.just_pressed(KeyCode::Delete)
            || keyboard_input.just_pressed(KeyCode::Backspace)
        {
            if summoned_minions.remove_summon(x, y) {
                for (entity, summon) in summoned_entities.iter() {
                    if summon.x == x && summon.y == y {
                        commands.entity(entity).despawn();
                    };
                }
            }
        }
    }
}

const DEBUG_SUMMONS: [&str; 3] = ["Skeleton", "Angel", "Watcher"];

pub fn place_summon(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    board_mouse_state: Res<BoardMouseState>,
    known_summons: Res<KnownSummons>,
    mut summoned: ResMut<SummonedMinions>,
) {
    if let Some((x, y)) = board_mouse_state.hovered_tile {
        if summoned.has_spawn_location(x, y) {
            return;
        }
        for (i, key) in [
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
            KeyCode::Numpad6,
            KeyCode::Numpad7,
            KeyCode::Numpad8,
            KeyCode::Numpad9,
        ]
        .iter()
        .enumerate()
        {
            if i >= known_summons.length() as usize {
                break;
            }
            if keyboard_input.just_pressed(*key) {
                let summon = known_summons.get(&DEBUG_SUMMONS[i].to_string());
                spawn_summon(&mut commands, &textures, summon.clone(), x, y, false);
                summoned.add_summon(summon, x, y);
            }
        }
    }
}

pub fn spawn_summon(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    summon_type: SummonType,
    x: usize,
    y: usize,
    real: bool,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                texture: textures.board.clone(),
                atlas: TextureAtlas {
                    index: summon_type.sprite_idx(),
                    layout: textures.board_layout.clone(),
                },
                sprite: Sprite {
                    color: if real {
                        Color::rgb(1., 1., 1.)
                    } else {
                        Color::rgb(0.8, 0.8, 1.)
                    },
                    ..Default::default()
                },
                transform: Transform::from_translation(
                    tile_position_to_translation(x as i32, y as i32).extend(1.),
                ),
                ..Default::default()
            },
            Summon {
                summon_type,
                time: 0.0,
                x,
                y,
            },
        ))
        .id()
}
