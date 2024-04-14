use prelude::*;

mod battle;
mod board;
mod bt;
mod flow;
mod loading;
mod menu;
mod prelude;
mod state;
mod summoner;
mod summons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.6, 0.6, 1.)))
            .add_plugins(EcssPlugin::with_hot_reload())
            .add_plugins(menu::MenuPlugin)
            .add_plugins(loading::LoadingPlugin)
            .add_plugins(board::BoardPlugin)
            .add_plugins(battle::BattlePlugin)
            .add_plugins(summoner::SummonerPlugin)
            .add_plugins(flow::StoryPlugin)
            .init_state::<GameState>()
            .add_systems(PostUpdate, force_stylesheet_refresh);
    }
}

fn force_stylesheet_refresh(mut query: Query<&mut StyleSheet>) {
    for mut style_sheet in query.iter_mut() {
        style_sheet.refresh();
    }
}
