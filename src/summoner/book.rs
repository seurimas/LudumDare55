use crate::prelude::*;

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
    texture_assets: &TextureAssets,
    summon: &SummonType,
    x: f32,
    y: f32,
    size: f32,
) -> Entity {
    spawner
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(size),
                height: Val::Px(size),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(size / 2.),
                        height: Val::Px(size / 2.),
                        ..Default::default()
                    },
                    image: UiImage::new(texture_assets.board.clone()),
                    z_index: ZIndex::Local(10),
                    ..Default::default()
                },
                TextureAtlas {
                    index: summon.sprite_idx(),
                    layout: texture_assets.board_layout.clone(),
                },
            ));
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(size),
                    height: Val::Px(size),
                    ..Default::default()
                },
                z_index: ZIndex::Local(5),
                image: UiImage::new(texture_assets.summon.clone()),
                ..Default::default()
            });
        })
        .id()
}
