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
    Aura {
        effect: AuraEffect,
        target: (usize, usize),
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorController {
    pub actions: Vec<Action>,
    pub picked_location: Option<(usize, usize)>,
    pub picked_index: Option<usize>,
    pub picked_aura: Option<AuraEffect>,
}

impl BehaviorController {
    pub fn remaining_stamina(&self, model: &BehaviorModel) -> i32 {
        let mut stamina = model.stats.stamina as i32;
        for action in &self.actions {
            match action {
                Action::Move { movement, .. } => stamina -= movement.stamina_cost,
                Action::Attack { attack, .. } => stamina -= attack.stamina_cost as i32,
                _ => {}
            }
        }
        stamina
    }
}

pub type CharacterBrainNode =
    dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController> + Send + Sync;
pub type CharacterBrainDef = UnpoweredTreeDef<SummonBehaviors, SummonWrapperDef>;

#[derive(Component)]
pub struct CharacterBrain {
    pub tree: Box<CharacterBrainNode>,
}

#[derive(Component)]
pub struct DeathCharacterBrain(pub CharacterBrain);

impl CharacterBrain {
    pub fn new(tree_def: &CharacterBrainDef) -> Self {
        let tree = tree_def.create_tree();
        CharacterBrain { tree }
    }
}

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
    PickAura,
    PickRandomAura,
    AttackTarget,
    MoveTowardsTarget,
    MoveAwayFromTarget,
    RefreshAuraForTarget,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TypePath)]
pub enum SummonWrapperDef {
    ForAllAllies,
    ForAllEnemies,
    ForAllAlliesInRange(i32),
    ForAllEnemiesInRange(i32),
}

pub enum SummonWrapper {
    ForAllAllies(Box<CharacterBrainNode>),
    ForAllEnemies(Box<CharacterBrainNode>),
    ForAllAlliesInRange(i32, Box<CharacterBrainNode>),
    ForAllEnemiesInRange(i32, Box<CharacterBrainNode>),
}

impl UnpoweredFunction for SummonWrapper {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            SummonWrapper::ForAllAllies(node) => {
                for ally in &model.allies {
                    controller.picked_location = Some(*ally);
                    match node.resume_with(model, controller) {
                        UnpoweredFunctionState::Complete => {}
                        _ => return UnpoweredFunctionState::Failed,
                    }
                }
                UnpoweredFunctionState::Complete
            }
            SummonWrapper::ForAllEnemies(node) => {
                for enemy in &model.enemies {
                    controller.picked_location = Some(*enemy);
                    match node.resume_with(model, controller) {
                        UnpoweredFunctionState::Complete => {}
                        _ => return UnpoweredFunctionState::Failed,
                    }
                }
                UnpoweredFunctionState::Complete
            }
            SummonWrapper::ForAllAlliesInRange(range, node) => {
                for ally in &model.allies {
                    let dx = ally.0 as i32 - model.position.0 as i32;
                    let dy = ally.1 as i32 - model.position.1 as i32;
                    if dx.abs() + dy.abs() <= *range {
                        controller.picked_location = Some(*ally);
                        match node.resume_with(model, controller) {
                            UnpoweredFunctionState::Complete => {}
                            _ => return UnpoweredFunctionState::Failed,
                        }
                    }
                }
                UnpoweredFunctionState::Complete
            }
            SummonWrapper::ForAllEnemiesInRange(range, node) => {
                for enemy in &model.enemies {
                    let dx = enemy.0 as i32 - model.position.0 as i32;
                    let dy = enemy.1 as i32 - model.position.1 as i32;
                    if dx.abs() + dy.abs() <= *range {
                        controller.picked_location = Some(*enemy);
                        match node.resume_with(model, controller) {
                            UnpoweredFunctionState::Complete => {}
                            _ => return UnpoweredFunctionState::Failed,
                        }
                    }
                }
                UnpoweredFunctionState::Complete
            }
        }
    }

    fn reset(&mut self, _parameter: &Self::Model) {}
}

impl UserWrapperDefinition<SummonBehaviors> for SummonWrapperDef {
    fn create_node_and_wrap(
        &self,
        mut nodes: Vec<
            Box<
                dyn UnpoweredFunction<
                        Model = <SummonBehaviors as UserNodeDefinition>::Model,
                        Controller = <SummonBehaviors as UserNodeDefinition>::Controller,
                    > + Send
                    + Sync,
            >,
        >,
    ) -> Box<
        dyn UnpoweredFunction<
                Model = <SummonBehaviors as UserNodeDefinition>::Model,
                Controller = <SummonBehaviors as UserNodeDefinition>::Controller,
            > + Send
            + Sync,
    > {
        match self {
            SummonWrapperDef::ForAllAllies => {
                Box::new(SummonWrapper::ForAllAllies(nodes.pop().unwrap()))
            }
            SummonWrapperDef::ForAllEnemies => {
                Box::new(SummonWrapper::ForAllEnemies(nodes.pop().unwrap()))
            }
            SummonWrapperDef::ForAllAlliesInRange(range) => Box::new(
                SummonWrapper::ForAllAlliesInRange(*range, nodes.pop().unwrap()),
            ),
            SummonWrapperDef::ForAllEnemiesInRange(range) => Box::new(
                SummonWrapper::ForAllEnemiesInRange(*range, nodes.pop().unwrap()),
            ),
        }
    }
}

impl UnpoweredFunction for SummonBehaviors {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
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
            SummonBehaviors::PickAura => {
                controller.picked_aura = model.stats.auras.get(0).cloned();
                UnpoweredFunctionState::Complete
            }
            SummonBehaviors::PickRandomAura => {
                if let Some(aura) = model.stats.auras.choose(&mut rand::thread_rng()) {
                    controller.picked_aura = Some(aura.clone());
                    UnpoweredFunctionState::Complete
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
                if let Some((t_x, t_y)) = controller.picked_location {
                    if let Some(index) = controller.picked_index {
                        let movement = model.stats.movements.get(index).unwrap();
                        try_move_towards(model, t_x, t_y, controller, movement);
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::MoveAwayFromTarget => {
                if let Some((t_x, t_y)) = controller.picked_location {
                    if let Some(index) = controller.picked_index {
                        let movement = model.stats.movements.get(index).unwrap();
                        let x = model.position.0;
                        let y = model.position.1;
                        let dx = t_x as i32 - model.position.0 as i32;
                        let dy = t_y as i32 - model.position.1 as i32;
                        if dx > 0 && x > 0 {
                            try_move_towards(model, 0, y, controller, movement);
                        } else if dx < 0 && x < 8 - 1 {
                            try_move_towards(model, 8 - 1, y, controller, movement);
                        } else if dy > 0 && y > 0 {
                            try_move_towards(model, x, 0, controller, movement);
                        } else if dy < 0 && y < 8 - 1 {
                            try_move_towards(model, x, 8 - 1, controller, movement);
                        }
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SummonBehaviors::RefreshAuraForTarget => {
                if let Some(effect) = &controller.picked_aura {
                    if let Some((x, y)) = controller.picked_location {
                        controller.actions.push(Action::Aura {
                            effect: effect.clone(),
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

fn try_move_towards(
    model: &BehaviorModel,
    t_x: usize,
    t_y: usize,
    controller: &mut BehaviorController,
    movement: &Movement,
) {
    let x = model.position.0;
    let y = model.position.1;
    let dx = t_x as i32 - model.position.0 as i32;
    let dy = t_y as i32 - model.position.1 as i32;
    if dx.abs() > dy.abs() {
        if model.location_occupied((x as i32 + dx.signum()) as usize, y) {
            controller.actions.push(Action::Move {
                movement: movement.clone(),
                target: (x, (y as i32 + dy) as usize),
            });
        } else {
            controller.actions.push(Action::Move {
                movement: movement.clone(),
                target: ((x as i32 + dx) as usize, y),
            });
        }
    } else {
        if model.location_occupied(x, (y as i32 + dy.signum()) as usize) {
            controller.actions.push(Action::Move {
                movement: movement.clone(),
                target: ((x as i32 + dx) as usize, y),
            });
        } else {
            controller.actions.push(Action::Move {
                movement: movement.clone(),
                target: (x, (y as i32 + dy) as usize),
            });
        }
    }
}
