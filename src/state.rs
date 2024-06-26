use crate::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Summoning,
    Battling,
    Looting,
    // Game over animations
    Defeat,
    Victory,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}
