use crate::prelude::*;

#[derive(Resource, Default)]
pub struct TurnOrder {
    pub order: Vec<Entity>,
}

#[derive(Resource)]
pub struct BattleSpeed(pub f32);

impl Default for BattleSpeed {
    fn default() -> Self {
        Self(0.25)
    }
}

pub fn prune_dead_entities(mut commands: Commands, query: Query<(Entity, &CharacterStats)>) {
    for (entity, stats) in query.iter() {
        if stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn prune_turn_order(
    mut turn_order: ResMut<TurnOrder>,
    query: Query<(Entity, &CharacterStats)>,
) {
    turn_order.order.retain(|entity| {
        if let Ok((_entity, stats)) = query.get(*entity) {
            stats.health > 0
        } else {
            false
        }
    });
}

pub fn end_battle(
    mut next_state: ResMut<NextState<GameState>>,
    fighters: Query<(Entity, &Faction, &Summon, &CharacterStats)>,
) {
    let mut player_units = 0;
    let mut enemy_units = 0;
    for (_entity, faction, _summon, stats) in fighters.iter() {
        match faction {
            Faction::Player => {
                player_units += 1;
            }
            Faction::Enemy => {
                enemy_units += 1;
            }
        }
    }
    if player_units == 0 || enemy_units == 0 {
        next_state.0 = Some(GameState::Looting);
    }
}

pub fn run_battle(
    mut ticker: Local<f32>,
    battle_speed: Res<BattleSpeed>,
    time: Res<Time>,
    mut commands: Commands,
    mut turn_order: ResMut<TurnOrder>,
    mut fighters: Query<(
        Entity,
        &Faction,
        &mut Summon,
        &mut CharacterStats,
        &mut CharacterBrain,
    )>,
) {
    *ticker += time.delta_seconds();
    if *ticker < battle_speed.0 {
        return;
    }
    if turn_order.order.is_empty() {
        for (entity, _, _, mut stats, _) in fighters.iter_mut() {
            turn_order.order.push(entity);
            stats.stamina += stats.stamina_regen;
        }
        turn_order
            .order
            .sort_by_cached_key(|_| rand::random::<u32>());
    }
    *ticker = 0.;
    let mut player_units = vec![];
    let mut enemy_units = vec![];
    for (_entity, faction, summon, _stats, _brain) in fighters.iter() {
        match faction {
            Faction::Player => player_units.push((summon.x, summon.y)),
            Faction::Enemy => enemy_units.push((summon.x, summon.y)),
        }
    }
    if player_units.is_empty() || enemy_units.is_empty() {
        return;
    }
    let next_turn = turn_order.order.pop().unwrap();
    let mut attacks = vec![];
    if let Ok((entity, faction, mut summon, mut stats, mut brain)) = fighters.get_mut(next_turn) {
        if stats.health == 0 {
            commands.entity(entity).despawn_recursive();
        }
        let model = BehaviorModel {
            position: (summon.x, summon.y),
            stats: stats.clone(),
            enemies: match faction {
                Faction::Player => enemy_units.clone(),
                Faction::Enemy => player_units.clone(),
            },
            allies: match faction {
                Faction::Player => player_units
                    .iter()
                    .filter(|(x, y)| *x != summon.x || *y != summon.y)
                    .cloned()
                    .collect(),
                Faction::Enemy => enemy_units
                    .iter()
                    .filter(|(x, y)| *x != summon.x || *y != summon.y)
                    .cloned()
                    .collect(),
            },
        };
        let mut controller = BehaviorController {
            actions: vec![],
            picked_location: None,
            picked_index: None,
        };
        brain.tree.resume_with(&model, &mut controller);
        for action in controller.actions {
            match action {
                Action::Move { movement, target } => {
                    let next_location =
                        movement.next_location(summon.x, summon.y, target.0, target.1);
                    if !model.location_occupied(next_location.0, next_location.1) {
                        summon.x = next_location.0;
                        summon.y = next_location.1;
                        stats.stamina -= movement.stamina_cost;
                    }
                }
                Action::Attack { attack, target } => {
                    attacks.push((entity, attack, target));
                }
            }
        }
    }
    for (entity, attack, target) in attacks {
        if let Ok((_entity, _faction, _summon, mut stats, _brain)) = fighters.get_mut(entity) {
            if stats.stamina >= attack.stamina_cost {
                stats.stamina -= attack.stamina_cost;
            }
        }
        if let Some((_entity, _faction, _summon, mut stats, _brain)) = fighters
            .iter_mut()
            .find(|(_, _, summon, _, _)| summon.x == target.0 && summon.y == target.1)
        {
            info!("Attacking {:?}", stats);
            stats.health -= attack.damage;
        }
    }
}
