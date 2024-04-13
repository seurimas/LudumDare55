use prelude::*;

mod battle;
mod board;
mod bt;
mod loading;
mod menu;
mod prelude;
mod state;
mod summons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(menu::MenuPlugin)
            .add_plugins(loading::LoadingPlugin)
            .add_plugins(board::BoardPlugin)
            .add_plugins(summons::SummonsPlugin)
            .add_plugins(battle::BattlePlugin)
            .init_state::<GameState>();
    }
}
