use crate::{prelude::*, summoner::spawn_summon_button};

pub fn setup_loot_screen(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_sections(vec![TextSection {
                    value: "Select a new summon!".to_string(),
                    style: TextStyle {
                        font: Default::default(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                }]),
                ..Default::default()
            });
            spawn_summon_button(parent, &texture_assets, &SummonType::debug(), 50., 50., 50.);
        });
}
