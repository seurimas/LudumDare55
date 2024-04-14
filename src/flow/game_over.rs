use crate::{
    board::{BorderTile, Tile},
    persistence::add_save_button,
    prelude::*,
};

pub fn check_for_game_over(
    mut commands: Commands,
    story_beat: Res<StoryBeat>,
    mut next_state: ResMut<NextState<GameState>>,
    sounds: Res<AudioAssets>,
    styles: Res<StyleAssets>,
    textures: Res<TextureAssets>,
    summons: Query<Entity, With<Summon>>,
) {
    if story_beat.victory {
        next_state.set(GameState::Victory);
        commands.spawn(AudioBundle {
            source: sounds.game_over_victory.clone(),
            ..Default::default()
        });
        spawn_overlay(&mut commands, true, &styles, &textures);
        for entity in summons.iter() {
            commands.entity(entity).despawn_recursive();
        }
    } else if story_beat.defeat {
        next_state.set(GameState::Defeat);
        commands.spawn(AudioBundle {
            source: sounds.game_over_defeat.clone(),
            ..Default::default()
        });
        spawn_overlay(&mut commands, false, &styles, &textures);
        for entity in summons.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_overlay(
    commands: &mut Commands,
    victory: bool,
    styles: &StyleAssets,
    textures: &TextureAssets,
) {
    commands
        .spawn((
            NodeBundle::default(),
            Class::new("game_over__parent"),
            StyleSheet::new(styles.game_over.clone()),
            GameOverActor::Overlay(victory),
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle::default(), Class::new("game_over")))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(textures.narration.clone()),
                            ..Default::default()
                        },
                        Class::new("game_over__bg"),
                    ));
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                if victory {
                                    "Victory!".to_string()
                                } else {
                                    "Defeat!".to_string()
                                },
                                TextStyle {
                                    font: Default::default(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        },
                        Class::new("game_over__text"),
                        GameOverActor::Text,
                    ));
                });
            if victory {
                add_save_button(parent);
            }
        });
}

#[derive(Component)]
pub enum GameOverActor {
    Bones,
    Ally,
    Overlay(bool),
    Text,
}

pub fn animate_game_over_defeat(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &GameOverActor), With<Summon>>,
    mut board_tiles: Query<(
        Option<&mut Tile>,
        Option<&mut BorderTile>,
        &mut TextureAtlas,
    )>,
    textures: Res<TextureAssets>,
    summons: Res<SummonsAssets>,
    summon_type: Res<Assets<SummonType>>,
) {
    let mut living = 0;
    let mut at_top = 0;
    for (entity, mut transform, actor) in query.iter_mut() {
        transform.translation.y -= 2. * TILE_SIZE * time.delta_seconds();
        let (x, y) = translation_to_tile_position(transform.translation.truncate());
        if y < 0 {
            commands.entity(entity).despawn_recursive();
        } else {
            if y >= 6 {
                at_top += 1;
            }
            living += 1;
            if y <= 3 {
                for (m_tile, m_border, mut atlas) in board_tiles.iter_mut() {
                    if let Some(mut tile) = m_tile {
                        if tile.x as i32 == x && tile.y as i32 == y {
                            if tile.sprite == 0 || tile.sprite == 1 {
                                tile.sprite += 16;
                                atlas.index = tile.sprite;
                            }
                        }
                    } else if let Some(mut border) = m_border {
                        if border.x as i32 == x && (border.sprite == 8 || border.sprite == 9) {
                            border.sprite += 16;
                            atlas.index = border.sprite;
                        }
                    }
                }
            }
        }
    }
    if living < 15 && at_top <= 1 {
        let x = (random::<f32>() * 8.) as usize;
        let y = 8;
        let summon_handle = summons.npc_summons.get("Bones").unwrap();
        let summon_type = summon_type.get(summon_handle).unwrap().clone();
        let summoned = spawn_summon(&mut commands, &textures, summon_type, x, y, true);
        commands.entity(summoned).insert(GameOverActor::Bones);
    }
}

pub fn animate_game_over_victory(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &GameOverActor), With<Summon>>,
    known_summons: Res<KnownSummons>,
    mut board_tiles: Query<(&mut Tile, &mut TextureAtlas)>,
    textures: Res<TextureAssets>,
    summons: Res<SummonsAssets>,
    summon_type: Res<Assets<SummonType>>,
) {
    let mut living = 0;
    let mut at_bottom = 0;
    for (entity, mut transform, actor) in query.iter_mut() {
        transform.translation.y += 2. * TILE_SIZE * time.delta_seconds();
        let (x, y) = translation_to_tile_position(transform.translation.truncate());
        if y >= 8 {
            commands.entity(entity).despawn_recursive();
        } else {
            if y >= 3 {
                for (mut tile, mut atlas) in board_tiles.iter_mut() {
                    if tile.x as i32 == x && tile.y as i32 == y {
                        if tile.sprite == 16 || tile.sprite == 17 {
                            tile.sprite -= 16;
                            atlas.index = tile.sprite;
                        }
                    }
                }
            } else {
                at_bottom += 1;
            }
            living += 1;
        }
    }
    if living < 15 && at_bottom <= 1 {
        let x = (random::<f32>() * 8.) as usize;
        let y = 0;
        let summon = known_summons.get_random().unwrap();
        let summoned = spawn_summon(&mut commands, &textures, summon, x, y, true);
        commands.entity(summoned).insert(GameOverActor::Ally);
    }
}
