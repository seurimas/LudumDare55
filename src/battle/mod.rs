use crate::prelude::*;

pub mod stats;
pub struct BattlePlugin;
pub use stats::*;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
