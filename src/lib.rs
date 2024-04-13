use prelude::*;

mod battle;
mod board;
mod bt;
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
            .init_state::<GameState>();
    }
}
