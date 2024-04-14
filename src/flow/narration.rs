use crate::prelude::*;

#[derive(Component)]
pub struct NarrationParent;
#[derive(Component)]
pub struct NarrationBox;

#[derive(Component)]
pub struct NarrationText(pub f32, pub String);

impl NarrationText {
    pub fn is_done(&self, full_string: &String) -> bool {
        self.1.len() >= full_string.len()
    }

    pub fn update(&mut self, delta: f32, full_string: &String) -> Option<usize> {
        if self.1.len() < full_string.len() {
            self.0 += delta;
            let index = (self.0 / 0.05) as usize;
            let changed = index != self.1.len();
            self.1 = full_string.chars().take(index).collect();
            if changed {
                Some(index)
            } else {
                None
            }
        } else {
            self.0 = 0.;
            None
        }
    }

    pub fn clear(&mut self) {
        self.0 = 0.;
        self.1 = "".to_string();
    }
}

pub fn spawn_narration_box(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(2),
                ..Default::default()
            },
            StyleSheet::new(styles.narration.clone()),
            Class::new("narration__parent"),
            NarrationParent,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle::default(),
                    Class::new("narration"),
                    Interaction::default(),
                    NarrationBox,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(texture_assets.narration.clone()),
                            z_index: ZIndex::Local(-1),
                            ..Default::default()
                        },
                        Class::new("narration__bg"),
                    ));
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "".to_string(),
                                TextStyle {
                                    font: Default::default(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            z_index: ZIndex::Local(3),
                            ..Default::default()
                        },
                        Class::new("narration__text"),
                        NarrationText(0., "".to_string()),
                    ));
                });
        });
}

pub fn update_narration_box(
    mut commands: Commands,
    mut query: Query<(&mut NarrationText, &mut Text)>,
    parent_query: Query<Entity, With<NarrationParent>>,
    time: Res<Time>,
    story_beat: Res<StoryBeat>,
    sounds: Res<AudioAssets>,
) {
    for (mut narrative, mut text) in query.iter_mut() {
        if let Some(active_narration) = story_beat.get_active_narration() {
            if let Some(change) = narrative.update(time.delta_seconds(), &active_narration) {
                if change % 2 == 0 {
                    commands.spawn(AudioBundle {
                        source: sounds.type_char.clone(),
                        ..Default::default()
                    });
                }
            }
            text.sections[0].value = narrative.1.clone();
        } else if narrative.1.len() > 0 {
            if let Some(parent) = parent_query.iter().next() {
                commands.entity(parent).despawn_recursive();
            }
        }
    }
}

pub fn advance_narration(
    mut commands: Commands,
    interactions: Query<(&Interaction, &NarrationBox), Changed<Interaction>>,
    parent_query: Query<Entity, With<NarrationParent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<&mut NarrationText>,
    mut story_beat: ResMut<StoryBeat>,
) {
    let mut advance_full = false;
    for (interaction, _) in interactions.iter() {
        match interaction {
            Interaction::Pressed => advance_full = true,
            _ => {}
        }
    }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        advance_full = true;
    }
    if advance_full {
        if let Some(active_narration) = story_beat.get_active_narration().cloned() {
            for mut text in text_query.iter_mut() {
                if text.is_done(&active_narration) {
                    text.clear();
                    story_beat.advance_narration();
                    if !story_beat.narrating() {
                        if let Some(parent) = parent_query.iter().next() {
                            commands.entity(parent).despawn_recursive();
                        }
                    }
                } else {
                    text.1 = active_narration.clone();
                }
            }
        } else {
            if let Some(parent) = parent_query.iter().next() {
                commands.entity(parent).despawn_recursive();
            }
        }
    }
}
