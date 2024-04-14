use crate::{persistence::add_load_button, prelude::*};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), hide_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    textures: Res<TextureAssets>,
    sounds: Res<AudioAssets>,
    query: Query<Entity, Without<Window>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.spawn((
        AudioBundle {
            source: sounds.welcome.clone(),
            settings: PlaybackSettings::LOOP,
        },
        Menu,
    ));
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle { ..default() },
            StyleSheet::new(styles.main_menu.clone()),
            Class::new("main_menu"),
            Menu,
        ))
        .with_children(|parent| {
            add_load_button(parent);
            let button_colors = ButtonColors::default();
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    Class::new("main_menu__play"),
                    ChangeState(GameState::Looting),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(textures.title.clone()),
                    z_index: ZIndex::Local(1),
                    ..Default::default()
                },
                Class::new("main_menu__title"),
            ));
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    story_asset: Res<Assets<Story>>,
    mut story: ResMut<Story>,
    summon_assets: Res<SummonsAssets>,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                    *story = story_asset
                        .get(summon_assets.story_teller.clone())
                        .unwrap()
                        .clone();
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn hide_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
