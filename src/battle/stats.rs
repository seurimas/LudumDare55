use crate::{prelude::*, summons::Tribe};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AuraEffect {
    Speed(String, i32, Vec<Tribe>),
    Attack(String, i32, Vec<Tribe>),
    Health(String, i32, Vec<Tribe>),
    Range(String, i32, Vec<Tribe>),
}

impl AuraEffect {
    pub fn name(&self) -> &str {
        match self {
            AuraEffect::Speed(name, _, _) => name,
            AuraEffect::Attack(name, _, _) => name,
            AuraEffect::Health(name, _, _) => name,
            AuraEffect::Range(name, _, _) => name,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CharacterStats {
    pub max_health: i32,
    pub health: i32,
    pub stamina: i32,
    pub stamina_regen: i32,
    pub tribe: Tribe,
    pub attacks: Vec<Attack>,
    pub movements: Vec<Movement>,
    pub auras: Vec<AuraEffect>,
    pub applied_auras: Vec<AuraEffect>,
}

impl CharacterStats {
    pub fn apply_aura(&mut self, aura: AuraEffect) {
        for applied_aura in self.applied_auras.iter() {
            if applied_aura == &aura {
                return;
            }
        }
        match &aura {
            AuraEffect::Speed(_, speed, tribes) => {
                if tribes.is_empty() || tribes.contains(&self.tribe) {
                    self.applied_auras.push(aura.clone());
                    for movement in self.movements.iter_mut() {
                        movement.tiles += speed;
                    }
                }
            }
            AuraEffect::Attack(_, bonus, tribes) => {
                if tribes.is_empty() || tribes.contains(&self.tribe) {
                    self.applied_auras.push(aura.clone());
                    for attack in self.attacks.iter_mut() {
                        attack.damage += bonus;
                    }
                }
            }
            AuraEffect::Health(_, health, tribes) => {
                if tribes.is_empty() || tribes.contains(&self.tribe) {
                    self.applied_auras.push(aura.clone());
                    self.max_health += health;
                    self.health += health;
                }
            }
            AuraEffect::Range(_, range, tribes) => {
                if tribes.is_empty() || tribes.contains(&self.tribe) {
                    self.applied_auras.push(aura.clone());
                    for attack in self.attacks.iter_mut() {
                        attack.range += range;
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Faction {
    Player,
    Enemy,
}
