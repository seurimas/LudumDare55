use bevy::utils::hashbrown::HashSet;

use crate::{prelude::*, summons::OverheadText};

use super::DeathCharacterBrain;

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

pub fn modify_battle_speed(keys: Res<ButtonInput<KeyCode>>, mut battle_speed: ResMut<BattleSpeed>) {
    if keys.pressed(KeyCode::Space) {
        battle_speed.0 = 1.;
    } else if keys.pressed(KeyCode::ShiftLeft)
        || keys.pressed(KeyCode::ShiftRight)
        || keys.pressed(KeyCode::Enter)
    {
        battle_speed.0 = 0.1;
    } else {
        battle_speed.0 = 0.25;
    }
}

pub fn end_battle(
    mut commands: Commands,
    my_minions: Res<SummonedMinions>,
    enemy_minions: Res<EnemyMinions>,
    mut next_state: ResMut<NextState<GameState>>,
    fighters: Query<(Entity, &Faction, &Summon, &CharacterStats)>,
    damage_text: Query<Entity, With<DamageText>>,
    mut story: ResMut<Story>,
    mut story_beat: ResMut<StoryBeat>,
    sounds: Res<AudioAssets>,
) {
    let mut player_units = my_minions.summons();
    let mut enemy_units = enemy_minions.0.summons();
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
        for entity in damage_text.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let beats = if enemy_units == 0 {
            commands.spawn(AudioBundle {
                source: sounds.victory_sting.clone(),
                ..Default::default()
            });
            story.win()
        } else {
            commands.spawn(AudioBundle {
                source: sounds.defeat_sting.clone(),
                ..Default::default()
            });
            story.lose()
        };
        story_beat.reset();
        story_beat.apply(beats);
        next_state.0 = Some(GameState::Looting);
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
}

#[derive(Resource, Default)]
pub struct BattleTimer(pub f32);

pub fn run_battle(
    mut ticker: ResMut<BattleTimer>,
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
        &mut DeathCharacterBrain,
    )>,
    mut attack_events: EventWriter<AttackEvent>,
) {
    ticker.0 += time.delta_seconds();
    if ticker.0 < battle_speed.0 {
        return;
    }
    if turn_order.order.is_empty() {
        for (entity, _, _, mut stats, _, _death) in fighters.iter_mut() {
            turn_order.order.push(entity);
            stats.stamina += stats.stamina_regen;
        }
        turn_order
            .order
            .sort_by_cached_key(|_| rand::random::<u32>());
    }
    ticker.0 = 0.;
    let mut player_units = vec![];
    let mut enemy_units = vec![];
    let mut dead_units = HashSet::new();
    for (_entity, faction, summon, stats, _brain, _death) in fighters.iter() {
        match faction {
            Faction::Player => player_units.push((summon.x, summon.y)),
            Faction::Enemy => enemy_units.push((summon.x, summon.y)),
        }
        if stats.health <= 0 {
            dead_units.insert((summon.x, summon.y));
        }
    }
    if player_units.is_empty() && enemy_units.is_empty() {
        return;
    }
    let next_turn = turn_order.order.pop().unwrap();
    let mut attacks = vec![];
    let mut auras = vec![];
    if let Ok((entity, faction, mut summon, mut stats, mut brain, mut death_brain)) =
        fighters.get_mut(next_turn)
    {
        if stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
        let model = BehaviorModel {
            position: (summon.x, summon.y),
            stats: stats.clone(),
            enemies: match faction {
                Faction::Player => enemy_units.clone(),
                Faction::Enemy => player_units.clone(),
            }
            .iter()
            .filter(|(x, y)| *x != summon.x || *y != summon.y && !dead_units.contains(&(*x, *y)))
            .cloned()
            .collect(),
            allies: match faction {
                Faction::Player => player_units,
                Faction::Enemy => enemy_units,
            }
            .iter()
            .filter(|(x, y)| *x != summon.x || *y != summon.y && !dead_units.contains(&(*x, *y)))
            .cloned()
            .collect(),
        };
        let mut controller = BehaviorController {
            actions: vec![],
            picked_location: None,
            picked_index: None,
            picked_aura: None,
        };
        if stats.health <= 0 {
            info!("Death brain!");
            death_brain.0.tree.resume_with(&model, &mut controller);
        } else {
            brain.tree.resume_with(&model, &mut controller);
        }
        for action in controller.actions {
            match action {
                Action::Move { movement, target } => {
                    for _ in 0..(movement.tiles) {
                        let next_location =
                            movement.next_location(summon.x, summon.y, target.0, target.1);
                        if !model.location_occupied(next_location.0, next_location.1) {
                            summon.x = next_location.0;
                            summon.y = next_location.1;
                            stats.stamina -= movement.stamina_cost;
                        }
                    }
                }
                Action::Attack { attack, target } => {
                    attacks.push((entity, attack, target));
                }
                Action::Aura { effect, target } => {
                    auras.push((effect, target));
                }
            }
        }
    }
    for (entity, attack, target) in attacks {
        if let Ok((_entity, _faction, _summon, mut stats, _brain, _death)) =
            fighters.get_mut(entity)
        {
            if stats.stamina >= attack.stamina_cost {
                stats.stamina -= attack.stamina_cost;
            }
        }
        if let Some((target, _faction, _summon, mut stats, _brain, _death)) = fighters
            .iter_mut()
            .find(|(_, _, summon, _, _, _)| summon.x == target.0 && summon.y == target.1)
        {
            attack_events.send(AttackEvent {
                attacker: entity,
                target,
                damage: attack.damage,
            });
            stats.health -= attack.damage;
        }
    }
    for (effect, target) in auras {
        if let Some((target, _faction, _summon, mut stats, _brain, _death)) = fighters
            .iter_mut()
            .find(|(_, _, summon, _, _, _)| summon.x == target.0 && summon.y == target.1)
        {
            stats.apply_aura(effect);
        }
    }
}

#[derive(Component)]
pub struct DamageText(pub f32);

pub fn animate_battle_text(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_query: Query<(Entity, &mut DamageText, &mut Transform)>,
) {
    for (entity, mut damage, mut transform) in damage_query.iter_mut() {
        damage.0 += time.delta_seconds();
        transform.translation.y += 20. * time.delta_seconds();
        if damage.0 > 1. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn animate_battle(
    mut commands: Commands,
    timer: Res<BattleTimer>,
    speed: Res<BattleSpeed>,
    time: Res<Time>,
    mut summon_query: Query<(&Summon, &mut Transform, &mut CharacterStats)>,
    mut attacks: EventReader<AttackEvent>,
    sounds: Res<AudioAssets>,
) {
    let t = time.delta_seconds() / (speed.0 - timer.0).max(0.0001).min(1.);
    for (summon, mut transform, mut stats) in summon_query.iter_mut() {
        let target = tile_position_to_translation(summon.x as i32, summon.y as i32);
        let translation = transform.translation.lerp(target.extend(1.), t);
        transform.translation = translation;
        if stats.health <= 0 {
            if !stats.is_dead {
                commands.spawn(AudioBundle {
                    source: sounds
                        .death_stings
                        .get(summon.summon_type.tribe.death_sting())
                        .unwrap()
                        .clone(),
                    ..Default::default()
                });
                stats.kill();
            }
            transform.scale = transform.scale.lerp(Vec3::splat(0.1), t);
        } else if transform.scale.max_element() < 1. {
            transform.scale += Vec3::splat(0.1);
        }
    }
    for attack in attacks.read() {
        if let Ok((_, mut transform, _)) = summon_query.get_mut(attack.target) {
            transform.scale = Vec3::splat(0.9);
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", attack.damage),
                        TextStyle {
                            font: Default::default(),
                            font_size: 16.0,
                            color: Color::rgb(1., 0., 0.),
                        },
                    ),
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0., 20., 2.),
                    ),
                    ..Default::default()
                },
                DamageText(0.0),
            ));
            commands.spawn(AudioBundle {
                source: sounds.hurt.clone(),
                ..Default::default()
            });
        }
        if let Ok((_, mut transform, _)) = summon_query.get_mut(attack.attacker) {
            transform.translation.y += 8.;
        }
    }
}

pub fn show_auras_overhead(
    stats: Query<(Entity, &CharacterStats, &Summon)>,
    mut overhead_query: Query<(&Parent, &mut Text), With<OverheadText>>,
) {
    for (parent, mut text) in overhead_query.iter_mut() {
        if let Ok((entity, stats, _summon)) = stats.get(parent.get()) {
            text.sections[0].value = stats
                .applied_auras
                .iter()
                .fold("".to_string(), |acc, aura| {
                    format!("{}\n{}", acc, aura.name())
                });
        }
    }
}
