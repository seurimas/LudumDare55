use crate::{prelude::*, summoner::spawn_summon_button};

pub fn setup_loot_screen(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn((
            NodeBundle::default(),
            StyleSheet::new(styles.loot.clone()),
            Name::new("loot"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: "Select a new summon!".to_string(),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }]),
                    ..Default::default()
                },
                Name::new("loot__text"),
            ));
            spawn_summon_button(parent, &texture_assets, &SummonType::debug(), 50., 50., 50.);
        });
}
