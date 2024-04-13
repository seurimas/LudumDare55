use crate::prelude::*;

use super::spawn_mana_bar;

#[derive(Resource, Default)]
pub struct KnownSummons {
    summons: HashMap<String, SummonType>,
}

impl KnownSummons {
    pub fn new(starting_summons: Vec<SummonType>) -> Self {
        let mut summons = HashMap::new();
        for summon in starting_summons {
            summons.insert(summon.name().to_string(), summon);
        }
        Self { summons }
    }
    pub fn get(&self, name: &String) -> SummonType {
        self.summons[name].clone()
    }

    pub fn length(&self) -> usize {
        self.summons.len()
    }
}

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
pub struct SummoningScroll;

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
            SummoningScroll,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle::default(), Class::new("summon_scroll")))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(texture_assets.scroll_back.clone()),
                            z_index: ZIndex::Local(-1),
                            ..Default::default()
                        },
                        Class::new("summon_scroll__bg"),
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

pub fn despawn_summoning_scroll(
    mut commands: Commands,
    query: Query<Entity, With<SummoningScroll>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
