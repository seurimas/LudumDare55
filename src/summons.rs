use crate::{battle::AuraEffect, prelude::*};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub enum Tribe {
    Angel,
    Undead,
    Fairy,
    Construct,
    Elemental,
    Demon,
    #[default]
    Enemy,
}

impl Tribe {
    pub fn tagline(&self) -> &str {
        match self {
            Tribe::Angel => "Angel - Powerful warriors with divine abilities.",
            Tribe::Undead => "Undead - Reanimated corpses with terrifying strength",
            Tribe::Fairy => "Fairy - Forces of nature and balance",
            Tribe::Construct => "Construct - Support Angel and Undead with auras",
            Tribe::Elemental => "Elemental - Support Angel and Fairy with boons upon dying",
            Tribe::Demon => "Demon - Support Undead and Fairy with powerful pacts",
            Tribe::Enemy => "Mysterious",
        }
    }

    pub fn sting(&self) -> &str {
        match self {
            Tribe::Angel => "angel_summon_sting",
            Tribe::Undead => "undead_summon_sting",
            Tribe::Fairy => "fairy_summon_sting",
            Tribe::Construct => "construct_summon_sting",
            Tribe::Elemental => "elemental_summon_sting",
            Tribe::Demon => "demon_summon_sting",
            Tribe::Enemy => "enemy_summon_sting",
        }
    }

    pub fn death_sting(&self) -> &str {
        match self {
            Tribe::Angel => "angel_death_sting",
            Tribe::Undead => "undead_death_sting",
            Tribe::Fairy => "fairy_death_sting",
            Tribe::Construct => "construct_death_sting",
            Tribe::Elemental => "elemental_death_sting",
            Tribe::Demon => "demon_death_sting",
            Tribe::Enemy => "enemy_death_sting",
        }
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
    #[serde(default)]
    auras: Vec<AuraEffect>,
    #[serde(default)]
    tagline: String,
    #[serde(default)]
    pub tribe: Tribe,
    brain: String,
    #[serde(default)]
    death_brain: String,
    short_code: String,
    prerequisites: (i32, Option<String>),
}

impl SummonType {
    pub fn debug() -> Self {
        Self {
            summon_name: "Debug".to_string(),
            sprite_idx: 7,
            mana_cost: 0,
            health: 1,
            stamina: 1,
            stamina_regen: 1,
            attacks: vec![Attack::debug()],
            movements: vec![Movement::debug()],
            auras: vec![],
            tagline: "You shouldn't see this".to_string(),
            tribe: Tribe::Enemy,
            short_code: "dbg".to_string(),
            brain: "fighter".to_string(),
            death_brain: "".to_string(),
            prerequisites: (0, None),
        }
    }

    pub fn name(&self) -> &str {
        &self.summon_name
    }

    pub fn short_code(&self) -> &str {
        &self.short_code
    }

    pub fn get_brain(&self, brain_assets: &BrainAssets) -> Option<Handle<CharacterBrainDef>> {
        brain_assets.brains.get(&*self.brain.as_str()).cloned()
    }

    pub fn get_death_brain(&self, brain_assets: &BrainAssets) -> Option<Handle<CharacterBrainDef>> {
        brain_assets
            .brains
            .get(&*self.death_brain.as_str())
            .cloned()
    }

    pub fn sprite_idx(&self) -> usize {
        self.sprite_idx
    }

    pub fn mana_cost(&self) -> i32 {
        self.mana_cost
    }

    pub fn prerequisites(&self) -> (i32, Option<String>) {
        self.prerequisites.clone()
    }

    pub fn descriptor(&self) -> Vec<TextSection> {
        vec![
            TextSection {
                value: self.summon_name.clone(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: "\n".to_string(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: self.tagline.clone(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: "\n".to_string(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: self.tribe.tagline().to_string(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: "\n".to_string(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: format!("Health: {}", self.health),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: "\n".to_string(),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
            TextSection {
                value: format!("Mana Cost: {}", self.mana_cost),
                style: TextStyle {
                    font: Default::default(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            },
        ]
    }
}

impl Into<CharacterStats> for SummonType {
    fn into(self) -> CharacterStats {
        CharacterStats {
            is_dead: false,
            max_health: self.health,
            health: self.health,
            stamina: self.stamina,
            stamina_regen: self.stamina_regen,
            attacks: self.attacks,
            movements: self.movements,
            name: self.summon_name,
            tribe: self.tribe,
            auras: self.auras,
            applied_auras: vec![],
        }
    }
}

#[derive(Component)]
pub struct Summon {
    pub summon_type: SummonType,
    pub time: f32,
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct OverheadText(pub f32);

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
                summon_type: summon_type.clone(),
                time: 0.0,
                x,
                y,
            },
            Into::<CharacterStats>::into(summon_type),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "".to_string(),
                        TextStyle {
                            font: Default::default(),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0., 20., 0.)),
                    ..Default::default()
                },
                OverheadText(0.),
            ));
        })
        .id()
}
