pub use crate::battle::{
    Action, Attack, BehaviorController, BehaviorModel, CharacterBrain, CharacterBrainDef,
    CharacterStats, Faction, Movement,
};
pub use crate::board::BoardMouseState;
pub use crate::bt::*;
pub use crate::loading::{AudioAssets, BrainAssets, StyleAssets, SummonsAssets, TextureAssets};
pub use crate::state::GameState;
pub use crate::summoner::{KnownSummons, SummonedMinions};
pub use crate::summons::{spawn_summon, Summon, SummonType};
pub use bevy::prelude::*;
pub use bevy::utils::HashMap;
pub use bevy_asset_loader::prelude::*;
pub use bevy_ecss::prelude::*;
pub use rand::prelude::*;
pub use serde::{Deserialize, Serialize};

pub use std::f32::consts::PI;
pub const WINDOW_SIZE: (f32, f32) = (948., 533.);
pub const TILE_SIZE: f32 = 32.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
pub const BOARD_SIZE: f32 = 8.0;

pub fn tile_position_to_translation(x: i32, y: i32) -> Vec2 {
    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
}
