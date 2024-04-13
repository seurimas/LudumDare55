use crate::prelude::*;

mod book;
mod mana;
pub use book::*;
pub use mana::*;

pub struct SummonerPlugin;

impl Plugin for SummonerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mana>();
    }
}
