use crate::prelude::*;

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
            brain: "fighter".to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.summon_name
    }

    pub fn get_brain(&self, brain_assets: &BrainAssets) -> Option<Handle<CharacterBrainDef>> {
        brain_assets.brains.get(&*self.brain.as_str()).cloned()
    }

    pub fn sprite_idx(&self) -> usize {
        self.sprite_idx
    }

    pub fn mana_cost(&self) -> i32 {
        self.mana_cost
    }
}

impl Into<CharacterStats> for SummonType {
    fn into(self) -> CharacterStats {
        CharacterStats {
            max_health: self.health,
            health: self.health,
            stamina: self.stamina,
            stamina_regen: self.stamina_regen,
            attacks: self.attacks,
            movements: self.movements,
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        summon_type.name().to_string(),
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
