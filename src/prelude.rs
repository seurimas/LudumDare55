pub use crate::board::BoardMouseState;
pub use crate::loading::{AudioAssets, SummonsAssets, TextureAssets};
pub use crate::state::GameState;
pub use crate::summons::SummonType;
pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use serde::{Deserialize, Serialize};

pub use std::f32::consts::PI;
pub const TILE_SIZE: f32 = 32.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

pub fn tile_position_to_translation(x: i32, y: i32) -> Vec2 {
    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
}
