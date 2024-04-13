use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorModel {
    pub position: (usize, usize),
    pub stats: CharacterStats,
    pub enemies: Vec<(usize, usize)>,
    pub allies: Vec<(usize, usize)>,
}

impl BehaviorModel {
    pub fn find_nearest_enemy(&self) -> Option<(usize, usize)> {
        self.enemies
            .iter()
            .min_by_key(|(x, y)| {
                let dx = *x as i32 - self.position.0 as i32;
                let dy = *y as i32 - self.position.1 as i32;
                dx * dx + dy * dy
            })
            .cloned()
    }

    pub fn find_nearest_ally(&self) -> Option<(usize, usize)> {
        self.allies
            .iter()
            .min_by_key(|(x, y)| {
                let dx = *x as i32 - self.position.0 as i32;
                let dy = *y as i32 - self.position.1 as i32;
                dx * dx + dy * dy
            })
            .cloned()
    }

    pub fn location_occupied(&self, x: usize, y: usize) -> bool {
        self.enemies.contains(&(x, y)) || self.allies.contains(&(x, y))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move {
        movement: Movement,
        target: (usize, usize),
    },
    Attack {
        attack: Attack,
        target: (usize, usize),
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorController {
    pub actions: Vec<Action>,
    pub picked_location: Option<(usize, usize)>,
    pub picked_index: Option<usize>,
}

impl BehaviorController {
    pub fn remaining_stamina(&self, model: &BehaviorModel) -> i32 {
        let mut stamina = model.stats.stamina as i32;
        for action in &self.actions {
            match action {
                Action::Move { movement, .. } => stamina -= movement.stamina_cost,
                Action::Attack { attack, .. } => stamina -= attack.stamina_cost as i32,
            }
        }
        stamina
    }
}

#[derive(Component)]
pub struct CharacterBrain {
    pub tree: Box<
        dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController> + Send + Sync,
    >,
}

impl CharacterBrain {
    pub fn new(tree_def: &CharacterBrainDef) -> Self {
        let tree = tree_def.create_tree();
        CharacterBrain { tree }
    }
}

pub type CharacterBrainDef = UnpoweredTreeDef<SummonBehaviors, ()>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TypePath)]
pub enum SummonBehaviors {
    FindNearestEnemy,
    FindNearestAlly,
    FindRandomEnemy,
    FindRandomAlly,
    PickValidAttack,
    PickRandomAttack,
    PickValidMovement,
    PickRandomMovement,
    AttackTarget,
    MoveTowardsTarget,
}

impl UnpoweredFunction for SummonBehaviors {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        info!("self: {:?}", self);
        match self {
            SummonBehaviors::FindNearestEnemy => {
                if let Some(enemy) = model.find_nearest_enemy() {
                    controller.picked_location = Some(enemy);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::FindNearestAlly => {
                if let Some(ally) = model.find_nearest_ally() {
                    controller.picked_location = Some(ally);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::FindRandomEnemy => {
                if let Some(enemy) = model.enemies.choose(&mut rand::thread_rng()) {
                    controller.picked_location = Some(*enemy);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::FindRandomAlly => {
                if let Some(ally) = model.allies.choose(&mut rand::thread_rng()) {
                    controller.picked_location = Some(*ally);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::PickValidAttack => {
                if let Some((x, y)) = controller.picked_location {
                    let dx = x as i32 - model.position.0 as i32;
                    let dy = y as i32 - model.position.1 as i32;
                    let distance = dx.abs() + dy.abs();
                    for (index, attack) in model.stats.attacks.iter().enumerate() {
                        if distance <= attack.range as i32 {
                            controller.picked_index = Some(index);
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                    UnpoweredFunctionState::Failed
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::PickRandomAttack => {
                if let Some((x, y)) = controller.picked_location {
                    let dx = x as i32 - model.position.0 as i32;
                    let dy = y as i32 - model.position.1 as i32;
                    let distance = dx.abs() + dy.abs();
                    let mut valid_attacks = vec![];
                    for (index, attack) in model.stats.attacks.iter().enumerate() {
                        if distance <= attack.range as i32 {
                            valid_attacks.push(index);
                        }
                    }
                    if let Some(index) = valid_attacks.choose(&mut rand::thread_rng()) {
                        controller.picked_index = Some(*index);
                        return UnpoweredFunctionState::Complete;
                    }
                    UnpoweredFunctionState::Failed
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::PickValidMovement => {
                if let Some((x, y)) = controller.picked_location {
                    let stamina = controller.remaining_stamina(model);
                    for (index, movement) in model.stats.movements.iter().enumerate() {
                        if movement.stamina_cost <= stamina {
                            controller.picked_index = Some(index);
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                    UnpoweredFunctionState::Failed
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::PickRandomMovement => {
                if let Some((x, y)) = controller.picked_location {
                    let stamina = controller.remaining_stamina(model);
                    let mut valid_moves = vec![];
                    for (index, movement) in model.stats.movements.iter().enumerate() {
                        if movement.stamina_cost <= stamina {
                            valid_moves.push(index);
                        }
                    }
                    if let Some(index) = valid_moves.choose(&mut rand::thread_rng()) {
                        controller.picked_index = Some(*index);
                        return UnpoweredFunctionState::Complete;
                    }
                    UnpoweredFunctionState::Failed
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::AttackTarget => {
                if let Some((x, y)) = controller.picked_location {
                    if let Some(index) = controller.picked_index {
                        let attack = model.stats.attacks.get(index).unwrap();
                        controller.actions.push(Action::Attack {
                            attack: attack.clone(),
                            target: (x, y),
                        });
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::MoveTowardsTarget => {
                if let Some((x, y)) = controller.picked_location {
                    if let Some(index) = controller.picked_index {
                        let movement = model.stats.movements.get(index).unwrap();
                        controller.actions.push(Action::Move {
                            movement: movement.clone(),
                            target: (x, y),
                        });
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, _parameter: &Self::Model) {}
}
