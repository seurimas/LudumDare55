use crate::{prelude::*, summoner::spawn_summon_button};

#[derive(Component)]
pub struct LootScreen;

pub fn setup_loot_screen(
    mut commands: Commands,
    styles: Res<StyleAssets>,
    texture_assets: Res<TextureAssets>,
) {
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
                Class::new("loot__text"),
            ));
            parent
                .spawn((NodeBundle::default(), Class::new("loot__summons")))
                .with_children(|parent| {
                    for _ in 0..3 {
                        spawn_summon_button(parent, &styles, &texture_assets, &SummonType::debug());
                    }
                });
        });
}

pub fn cleanup_loot_screen(mut commands: Commands, query: Query<Entity, With<LootScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
