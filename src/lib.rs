use prelude::*;

mod loading;
mod prelude;
mod state;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .init_state::<GameState>();
    }
}
