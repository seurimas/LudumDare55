use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Attack {
    pub damage: i32,
    pub range: i32,
    pub stamina_cost: i32,
}

impl Attack {
    pub fn debug() -> Self {
        Self {
            damage: 1,
            range: 100,
            stamina_cost: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Movement {
    pub stamina_cost: i32,
    pub tiles: i32,
}

impl Movement {
    pub fn debug() -> Self {
        Self {
            stamina_cost: 1,
            tiles: 1,
        }
    }
}

impl Movement {
    pub fn next_location(
        &self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> (usize, usize) {
        let dx = end_x as i32 - start_x as i32;
        let dy = end_y as i32 - start_y as i32;
        if dx == 0 && dy == 0 {
            return (start_x, start_y);
        } else if dx.abs() > dy.abs() {
            if dx > 0 {
                (start_x + 1, start_y)
            } else {
                (start_x - 1, start_y)
            }
        } else {
            if dy > 0 {
                (start_x, start_y + 1)
            } else {
                (start_x, start_y - 1)
            }
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CharacterStats {
    pub max_health: i32,
    pub health: i32,
    pub stamina: i32,
    pub stamina_regen: i32,
    pub attacks: Vec<Attack>,
    pub movements: Vec<Movement>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Faction {
    Player,
    Enemy,
}
