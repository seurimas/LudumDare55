use crate::prelude::*;

#[derive(Resource)]
pub struct Mana {
    pub max_mana: i32,
    pub used_mana: i32,
}

impl Default for Mana {
    fn default() -> Self {
        Self {
            max_mana: 2,
            used_mana: 0,
        }
    }
}

impl Mana {
    pub fn mana_left(&self) -> i32 {
        self.max_mana - self.used_mana
    }
}

pub fn mana_bar_system(
    mana: Res<Mana>,
    mut fill_query: Query<(&ManaFill, &mut Style)>,
    mut text_query: Query<(&ManaText, &mut Text)>,
) {
    for (_, mut style) in fill_query.iter_mut() {
        style.width = Val::Percent((mana.used_mana as f32 / mana.max_mana as f32) * 100.);
    }
    for (_, mut text) in text_query.iter_mut() {
        text.sections[0].value = format!("Mana available: {}/{}", mana.mana_left(), mana.max_mana);
    }
}

pub fn mana_tally_system(mut mana: ResMut<Mana>, summoned_minions: Res<SummonedMinions>) {
    mana.used_mana = summoned_minions.mana();
}

#[derive(Component)]
pub struct ManaBar;

#[derive(Component)]
pub struct ManaFill;

#[derive(Component)]
pub struct ManaText;

pub fn spawn_mana_bar(parent: &mut ChildBuilder, styles: &StyleAssets) -> Entity {
    parent
        .spawn((NodeBundle::default(), Class::new("mana_bar"), ManaBar))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.),
                        height: Val::Percent(100.),
                        right: Val::Px(0.),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Class::new("mana_bar__fill"),
                ManaFill,
            ));
            parent.spawn((
                TextBundle {
                    text: Text::from_sections(vec![TextSection {
                        value: format!("Mana..."),
                        style: TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    }]),
                    ..Default::default()
                },
                Class::new("mana_bar__text"),
                ManaText,
            ));
        })
        .id()
}
