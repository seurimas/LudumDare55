use crate::prelude::*;

#[derive(Component)]
pub struct CharacterStats {
    pub health: u32,
    pub stamina: u32,
    pub attacks: Vec<(u32, u32)>,
    pub movements: Vec<(u32, u32)>,
}
