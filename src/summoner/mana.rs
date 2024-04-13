use crate::prelude::*;

#[derive(Resource)]
pub struct Mana {
    pub max_mana: i32,
}

impl Default for Mana {
    fn default() -> Self {
        Self { max_mana: 2 }
    }
}
