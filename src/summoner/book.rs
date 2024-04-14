use bevy::transform::commands;

use crate::prelude::*;

use super::spawn_mana_bar;

#[derive(Resource, Default)]
pub struct KnownSummons {
    summons: HashMap<String, SummonType>,
    hotkeys: HashMap<KeyCode, String>,
    summon_hotkeys: HashMap<String, KeyCode>,
    active: Option<String>,
}

pub const HOTKEYS: [KeyCode; 10] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
    KeyCode::Digit0,
];

impl KnownSummons {
    pub fn new(starting_summons: Vec<SummonType>) -> Self {
        let mut summons = HashMap::new();
        let mut hotkeys = HashMap::new();
        let mut summon_hotkeys = HashMap::new();
        for (idx, summon) in starting_summons.iter().enumerate() {
            hotkeys.insert(HOTKEYS[idx], summon.name().to_string());
            summon_hotkeys.insert(summon.name().to_string(), HOTKEYS[idx]);
            summons.insert(summon.name().to_string(), summon.clone());
        }
        Self {
            summons,
            hotkeys,
            summon_hotkeys,
            active: None,
        }
    }

    pub fn add(&mut self, summon: SummonType) {
        let name = summon.name().to_string();
        self.summons.insert(name.clone(), summon);
        if self.summons.len() <= HOTKEYS.len() {
            self.hotkeys
                .insert(HOTKEYS[self.summons.len() - 1], name.clone());
            self.summon_hotkeys
                .insert(name, HOTKEYS[self.summons.len() - 1]);
        }
    }

    pub fn has(&self, name: &String) -> bool {
        self.summons.contains_key(name)
    }

    pub fn get(&self, name: &String) -> SummonType {
        self.summons[name].clone()
    }

    pub fn get_by_hotkey(&self, key: KeyCode) -> Option<SummonType> {
        if let Some(name) = self.hotkeys.get(&key) {
            return Some(self.get(name));
        }
        None
    }

    pub fn get_hotkey(&self, name: &String) -> Option<KeyCode> {
        if let Some(key) = self.summon_hotkeys.get(name) {
            return Some(*key);
        }
        None
    }

    pub fn get_active(&self) -> Option<SummonType> {
        if let Some(active) = &self.active {
            return Some(self.get(active));
        }
        None
    }

    pub fn get_random(&self) -> Option<SummonType> {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.summons.len());
        self.summons.values().nth(idx).cloned()
    }

    pub fn length(&self) -> usize {
        self.summons.len()
    }
}

#[derive(Component)]
pub struct SummonButton(pub bool, pub SummonType);

#[derive(Component)]
pub struct SummonButtonHotkey;

pub fn spawn_summon_button(
    spawner: &mut ChildBuilder,
    styles: &StyleAssets,
    texture_assets: &TextureAssets,
    summon: &SummonType,
) -> Entity {
    spawner
        .spawn((
            NodeBundle::default(),
            StyleSheet::new(styles.summon_button.clone()),
            Class::new("summon_button"),
            SummonButton(false, summon.clone()),
            Interaction::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(texture_assets.summon.clone()),
                    z_index: ZIndex::Local(-1),
                    ..Default::default()
                },
                Class::new("summon_button__bg"),
            ));
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: summon.name().to_string(),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }]),
                    z_index: ZIndex::Local(1),
                    ..Default::default()
                },
                Class::new("summon_button__name"),
            ));
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                    }]),
                    z_index: ZIndex::Local(3),
                    ..Default::default()
                },
                Class::new("summon_button__hotkey"),
                SummonButtonHotkey,
            ));
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(texture_assets.board.clone()),
                    z_index: ZIndex::Local(1),
                    ..Default::default()
                },
                TextureAtlas {
                    index: summon.sprite_idx(),
                    layout: texture_assets.board_layout.clone(),
                },
                Class::new("summon_button__icon"),
            ));
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: format!("Mana: {}", summon.mana_cost()),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }]),
                    z_index: ZIndex::Local(1),
                    ..Default::default()
                },
                Class::new("summon_button__mana_cost"),
            ));
        })
        .id()
}

#[derive(Component)]
pub struct SummoningScrollParent;

#[derive(Component)]
pub struct SummoningScroll(pub f32);

#[derive(Component)]
pub struct SummoningScrollButton(SummonType);

const SCROLL_CLOSED: f32 = -479.0;
const SCROLL_OPEN: f32 = 0.0;
const SCOLL_OPEN_SPEED: f32 = 500.0;
const SCOLL_CLOSE_SPEED: f32 = 500.0;

impl SummoningScroll {
    pub fn scroll(&mut self, delta_seconds: f32, open: bool) -> f32 {
        if open {
            self.0 += delta_seconds * SCOLL_OPEN_SPEED;
        } else {
            self.0 -= delta_seconds * SCOLL_CLOSE_SPEED;
        }
        if self.0 > SCROLL_OPEN {
            self.0 = SCROLL_OPEN;
        } else if self.0 < SCROLL_CLOSED {
            self.0 = SCROLL_CLOSED;
        }
        self.0
    }
}

pub fn spawn_summoning_scroll(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    texture_assets: Res<TextureAssets>,
    known_summons: Res<KnownSummons>,
) {
    commands
        .spawn((
            NodeBundle::default(),
            StyleSheet::new(styles.summon_scroll.clone()),
            Class::new("summon_scroll__parent"),
            SummoningScrollParent,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle::default(),
                    Class::new("summon_scroll"),
                    SummoningScroll(SCROLL_CLOSED),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(texture_assets.scroll_back.clone()),
                            z_index: ZIndex::Local(-1),
                            ..Default::default()
                        },
                        Class::new("summon_scroll__bg"),
                    ));
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(texture_assets.scroll_side.clone()),
                            z_index: ZIndex::Local(3),
                            ..Default::default()
                        },
                        Class::new("summon_scroll__side"),
                    ));
                    parent
                        .spawn((NodeBundle::default(), Class::new("summon_scroll__summons")))
                        .with_children(|parent| {
                            for summon in known_summons.summons.values() {
                                spawn_summon_button(parent, &styles, &texture_assets, summon);
                            }
                        });
                    spawn_mana_bar(parent, &styles);
                });
        });
}

pub fn animate_summoning_scroll_opening(
    mut query: Query<(&mut SummoningScroll, &mut Style)>,
    time: Res<Time>,
) {
    for (mut scroll, mut style) in query.iter_mut() {
        style.right = Val::Px(scroll.scroll(time.delta_seconds(), true));
    }
}

pub fn animate_summoning_scroll_closing(
    mut commands: Commands,
    mut query: Query<(&mut SummoningScroll, &mut Style, &Parent)>,
    time: Res<Time>,
) {
    for (mut scroll, mut style, parent) in query.iter_mut() {
        style.right = Val::Px(scroll.scroll(time.delta_seconds(), false));
        if scroll.0 <= SCROLL_CLOSED {
            commands.entity(parent.get()).despawn_recursive();
        }
    }
}

pub fn show_hotkeys(
    mut query: Query<(&mut Text, &Parent), With<SummonButtonHotkey>>,
    buttons: Query<(Entity, &SummonButton)>,
    known_summons: Res<KnownSummons>,
) {
    for (mut text, parent) in query.iter_mut() {
        for (button, summon_button) in buttons.iter() {
            if parent.get() == button {
                if let Some(key) = known_summons
                    .hotkeys
                    .iter()
                    .find(|(_, value)| {
                        return value == &summon_button.1.name();
                    })
                    .map(|(key, _)| key)
                {
                    text.sections[0].value = match key {
                        KeyCode::Digit0 => "0".to_string(),
                        KeyCode::Digit1 => "1".to_string(),
                        KeyCode::Digit2 => "2".to_string(),
                        KeyCode::Digit3 => "3".to_string(),
                        KeyCode::Digit4 => "4".to_string(),
                        KeyCode::Digit5 => "5".to_string(),
                        KeyCode::Digit6 => "6".to_string(),
                        KeyCode::Digit7 => "7".to_string(),
                        KeyCode::Digit8 => "8".to_string(),
                        KeyCode::Digit9 => "9".to_string(),
                        _ => "".to_string(),
                    };
                } else {
                    text.sections[0].value = "".to_string();
                }
            }
        }
    }
}

pub fn handle_summon_button_interactions(
    mut known_summons: ResMut<KnownSummons>,
    mut query: Query<(&mut Class, &mut SummonButton, &Interaction)>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut class, mut button, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                known_summons.active = Some(button.1.name().to_string());
            }
            Interaction::Hovered => {
                class.add("hovered");
            }
            Interaction::None => {
                class.remove("hovered");
            }
        }
        if let Some(key) = known_summons.get_hotkey(&button.1.name().to_string()) {
            if key_input.just_pressed(key) {
                known_summons.active = Some(button.1.name().to_string());
            }
        }
        if let Some(active) = &known_summons.active {
            if active != &button.1.name() {
                button.0 = false;
            } else {
                button.0 = true;
            }
        } else {
            button.0 = false;
        }
        if button.0 {
            class.add("selected");
        } else {
            class.remove("selected");
        }
    }
}
