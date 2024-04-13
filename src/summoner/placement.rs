use crate::prelude::*;

use super::{mana, HOTKEYS};

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
        self.mana += summon.mana_cost();
        self.spawn_locations
            .insert((x, y), summon.name().to_string());
        self.mana_locations.insert((x, y), summon.mana_cost());
    }

    pub fn remove_summon(&mut self, x: usize, y: usize) -> bool {
        if let (Some(summon_name), Some(mana_cost)) = (
            self.spawn_locations.remove(&(x, y)),
            self.mana_locations.remove(&(x, y)),
        ) {
            self.mana -= mana_cost;
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

    pub fn mana(&self) -> i32 {
        self.mana
    }
}

pub fn animate_summons(time: Res<Time>, mut query: Query<(&mut Summon, &mut Transform)>) {
    for (mut summon, mut transform) in query.iter_mut() {
        summon.time += time.delta_seconds();
        let offset = summon.time.sin() + PI;
        let translation =
            tile_position_to_translation(summon.x as i32, summon.y as i32) + Vec2::new(0., offset);
        transform.translation = translation.extend(1.);
    }
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
                        commands.entity(entity).despawn_recursive();
                    };
                }
            }
        }
    }
}

pub fn place_summon(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    button_input: Res<ButtonInput<MouseButton>>,
    board_mouse_state: Res<BoardMouseState>,
    known_summons: Res<KnownSummons>,
    mut summoned: ResMut<SummonedMinions>,
    mana: Res<mana::Mana>,
) {
    if let Some((x, y)) = board_mouse_state.hovered_tile {
        if summoned.has_spawn_location(x, y) {
            return;
        }
        for (i, key) in HOTKEYS.iter().enumerate() {
            if i >= known_summons.length() as usize {
                break;
            }
            if keyboard_input.just_pressed(*key) {
                if known_summons.get_active() == known_summons.get_by_hotkey(*key) {
                    let summon = known_summons.get_active().unwrap();
                    if mana.mana_left() >= summon.mana_cost() {
                        spawn_summon(&mut commands, &textures, summon.clone(), x, y, false);
                        summoned.add_summon(summon, x, y);
                    }
                }
            }
        }
        if button_input.just_pressed(MouseButton::Left) {
            if let Some(summon) = known_summons.get_active() {
                if mana.mana_left() >= summon.mana_cost() {
                    spawn_summon(&mut commands, &textures, summon.clone(), x, y, false);
                    summoned.add_summon(summon, x, y);
                }
            }
        }
    }
}
