use crate::prelude::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.6, 0.6, 1.)))
            .add_plugins(EcssPlugin::default())
            .add_plugins(crate::menu::MenuPlugin)
            .add_plugins(crate::loading::LoadingPlugin)
            .add_plugins(crate::board::BoardPlugin)
            .add_plugins(crate::battle::BattlePlugin)
            .add_plugins(crate::summoner::SummonerPlugin)
            .add_plugins(crate::flow::StoryPlugin)
            .add_plugins(crate::persistence::PersistencePlugin)
            .init_state::<GameState>()
            .add_systems(PostUpdate, force_stylesheet_refresh);
    }
}

fn force_stylesheet_refresh(mut query: Query<&mut StyleSheet>) {
    for mut style_sheet in query.iter_mut() {
        style_sheet.refresh();
    }
}
