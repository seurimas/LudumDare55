use crate::prelude::*;

use super::SummonButton;

#[derive(Component)]
pub struct HelpOverlay;

#[derive(Component)]
pub struct HelpOverlayText;

pub fn spawn_help_overlay(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn((
            NodeBundle::default(),
            Class::new("help__parent"),
            StyleSheet::new(styles.help.clone()),
            HelpOverlay,
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
                        Class::new("help__bg"),
                    ));
                    parent.spawn((
                        TextBundle {
                            text: Text::from_sections(vec![]),
                            ..Default::default()
                        },
                        Class::new("help__text"),
                        HelpOverlayText,
                    ));
                });
        });
}

pub fn despawn_help_overlay(mut commands: Commands, query: Query<Entity, With<HelpOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn show_hovered_stats(
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    board_mouse_state: Res<BoardMouseState>,
    buttons: Query<(&SummonButton, &Interaction)>,
    mut overlay: Query<(&mut Visibility, &mut Style), With<HelpOverlay>>,
    mut text_query: Query<(&mut Text, &HelpOverlayText)>,
    stats_query: Query<(&Summon, &CharacterStats)>,
) {
    let mouse_position = match q_windows.iter().next() {
        Some(window) => window.cursor_position(),
        None => return,
    };
    let mut descriptor = None;
    if let Some((x, y)) = board_mouse_state.hovered_tile {
        if let Some((_summon, stats)) = stats_query
            .iter()
            .find(|(summon, _)| summon.x == x && summon.y == y)
        {
            descriptor = Some(stats.descriptor());
        }
    }
    for (button, interaction) in buttons.iter() {
        if *interaction == Interaction::Hovered {
            descriptor = Some(button.1.descriptor());
        }
    }
    if let Some(descriptor) = descriptor {
        for (mut text, _) in text_query.iter_mut() {
            text.sections = descriptor.clone();
        }
        for (mut visibility, mut style) in overlay.iter_mut() {
            *visibility = Visibility::Visible;
            if let Some(mouse_position) = mouse_position {
                if mouse_position.x + 240. > WINDOW_SIZE.0 {
                    style.right = Val::Px(WINDOW_SIZE.0 - mouse_position.x);
                    style.left = Val::Auto;
                    style.align_items = AlignItems::FlexEnd;
                } else {
                    style.left = Val::Px(mouse_position.x);
                    style.right = Val::Auto;
                    style.align_items = AlignItems::FlexStart;
                }
                style.top = Val::Px(mouse_position.y);
            }
        }
    } else {
        for (mut visibility, _) in overlay.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}
