use crate::{
    prelude::*,
    summoner::{spawn_summon_button, SummonButton},
};

#[derive(Component)]
pub struct LootScreen;

#[derive(Component)]
pub struct LootButton(pub SummonType);

#[derive(Component)]
pub struct LootDescriptor(pub Option<SummonType>);

pub fn setup_loot_screen(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    texture_assets: Res<TextureAssets>,
    summons_assets: Res<SummonsAssets>,
    known_summons: Res<KnownSummons>,
    assets_summon_types: Res<Assets<SummonType>>,
    mut mana: ResMut<Mana>,
    story_beat: Res<StoryBeat>,
) {
    if story_beat.mana_gained > 0 {
        mana.max_mana += story_beat.mana_gained;
    }
    let mut available_summons = summons_assets
        .player_summons
        .values()
        .filter(|summon| {
            let summon_type = assets_summon_types.get(*summon).unwrap();
            if known_summons.has(&summon_type.name().to_string()) {
                return false;
            }
            let prerequisites = summon_type.prerequisites();
            if prerequisites.1.is_none() {
                mana.max_mana >= prerequisites.0
            } else {
                known_summons.has(&prerequisites.1.unwrap()) && mana.max_mana >= prerequisites.0
            }
        })
        .collect::<Vec<_>>();
    let mut pickable_summons = vec![];
    for _ in 0..3 {
        if !available_summons.is_empty() {
            let idx = rand::thread_rng().gen_range(0..available_summons.len());
            pickable_summons.push(available_summons.remove(idx));
        }
    }
    let mut buttons = vec![];
    commands
        .spawn((
            NodeBundle::default(),
            StyleSheet::new(styles.loot.clone()),
            Class::new("loot"),
            LootScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "Select a new summon!\n".to_string(),
                            style: TextStyle {
                                font: Default::default(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: if story_beat.mana_gained > 0 {
                                format!(
                                    "You gained {} mana!\nYour new max is: {}",
                                    story_beat.mana_gained, mana.max_mana
                                )
                            } else {
                                format!("Your max mana is: {}", mana.max_mana)
                            },
                            style: TextStyle {
                                font: Default::default(),
                                font_size: 20.0,
                                color: Color::BLUE,
                            },
                        },
                    ]),
                    ..Default::default()
                },
                Class::new("loot__text"),
            ));
            parent
                .spawn((NodeBundle::default(), Class::new("loot__summons")))
                .with_children(|parent| {
                    for summon in pickable_summons {
                        let summon = assets_summon_types.get(summon).unwrap();
                        let button = spawn_summon_button(parent, &styles, &texture_assets, summon);
                        buttons.push((button, LootButton(summon.clone())));
                    }
                });
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }]),
                    ..Default::default()
                },
                Class::new("loot__text"),
                LootDescriptor(None),
            ));
        });
    for (button, component) in buttons {
        commands.entity(button).insert(component);
    }
}

pub fn handle_loot_button_click(
    mut state: ResMut<NextState<GameState>>,
    mut known_summons: ResMut<KnownSummons>,
    mut query: Query<
        (&mut Class, &mut SummonButton, &LootButton, &Interaction),
        Changed<Interaction>,
    >,
    mut descriptor_query: Query<(&mut Text, &mut LootDescriptor)>,
) {
    for (mut class, mut summon, loot, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if summon.0 {
                    state.set(GameState::Summoning);
                    known_summons.add(loot.0.clone());
                } else {
                    summon.0 = true;
                    class.add("selected");
                }
            }
            Interaction::Hovered => {
                class.add("hovered");
                for (mut text, mut descriptor) in descriptor_query.iter_mut() {
                    descriptor.0 = Some(loot.0.clone());
                    text.sections = loot.0.descriptor();
                }
            }
            Interaction::None => {
                summon.0 = false;
                class.remove("hovered");
                class.remove("selected");
            }
        }
    }
}

pub fn cleanup_loot_screen(mut commands: Commands, query: Query<Entity, With<LootScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
