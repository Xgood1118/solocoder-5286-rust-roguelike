pub mod behavior_tree;
pub mod boss_ai;

use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::map::map::GameMap;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ai_system,
                boss_phase_transition,
            )
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(CombatState::EnemyAction)),
        );
    }
}

#[derive(Component, Debug, Clone)]
pub struct AIState {
    pub current_state: AIBehaviorState,
    pub target: Option<Entity>,
    pub last_seen_position: Option<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AIBehaviorState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
    Heal,
    Summon,
    Special,
}

impl Default for AIState {
    fn default() -> Self {
        Self {
            current_state: AIBehaviorState::Idle,
            target: None,
            last_seen_position: None,
        }
    }
}

fn ai_system(
    mut commands: Commands,
    mut ai_query: Query<(Entity, &Position, &FieldOfView, &Stats, &mut AIState, &EnemyKind)>,
    player_query: Query<(Entity, &Position), With<Player>>,
    map: Res<GameMap>,
) {
    if let Ok((player_entity, player_pos)) = player_query.get_single() {
        let mut actions: Vec<(Entity, AIAction)> = Vec::new();

        for (entity, pos, fov, stats, mut ai_state, kind) in ai_query.iter_mut() {
            let can_see_player = fov.visible.iter().any(|p| p == player_pos);

            if can_see_player {
                ai_state.target = Some(player_entity);
                ai_state.last_seen_position = Some(*player_pos);
            }

            let action = decide_action(
                &ai_state,
                pos,
                stats,
                kind,
                player_pos,
                &map,
                can_see_player,
            );

            actions.push((entity, action));
        }

        for (entity, action) in actions {
            execute_action(&mut commands, entity, action, &map);
        }
    }
}

#[derive(Debug)]
enum AIAction {
    Move(Position),
    Attack(Entity),
    Wait,
    HealSelf,
    SummonMinions,
    SpecialAbility,
}

fn decide_action(
    ai_state: &AIState,
    pos: &Position,
    stats: &Stats,
    kind: &EnemyKind,
    player_pos: &Position,
    map: &GameMap,
    can_see_player: bool,
) -> AIAction {
    let distance = pos.distance(player_pos);

    if can_see_player {
        if distance <= 1.5 {
            if let Some(target) = ai_state.target {
                return AIAction::Attack(target);
            }
        }

        let next_step = get_next_step_towards(pos, player_pos, map);
        if let Some(step) = next_step {
            return AIAction::Move(step);
        }
    } else if let Some(last_seen) = ai_state.last_seen_position {
        if pos.distance(&last_seen) > 1.0 {
            if let Some(step) = get_next_step_towards(pos, &last_seen, map) {
                return AIAction::Move(step);
            }
        }
    }

    AIAction::Wait
}

fn get_next_step_towards(from: &Position, to: &Position, map: &GameMap) -> Option<Position> {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    let mut candidates = Vec::new();

    if dx.abs() > dy.abs() {
        if dx > 0 {
            candidates.push(Position::new(from.x + 1, from.y));
        } else if dx < 0 {
            candidates.push(Position::new(from.x - 1, from.y));
        }
        if dy > 0 {
            candidates.push(Position::new(from.x, from.y + 1));
        } else if dy < 0 {
            candidates.push(Position::new(from.x, from.y - 1));
        }
    } else {
        if dy > 0 {
            candidates.push(Position::new(from.x, from.y + 1));
        } else if dy < 0 {
            candidates.push(Position::new(from.x, from.y - 1));
        }
        if dx > 0 {
            candidates.push(Position::new(from.x + 1, from.y));
        } else if dx < 0 {
            candidates.push(Position::new(from.x - 1, from.y));
        }
    }

    for candidate in candidates {
        if map.in_bounds(candidate.x, candidate.y)
            && !map.is_occupied(candidate.x, candidate.y)
        {
            return Some(candidate);
        }
    }

    None
}

fn execute_action(commands: &mut Commands, entity: Entity, action: AIAction, map: &GameMap) {
    match action {
        AIAction::Move(dest) => {
            if !map.is_occupied(dest.x, dest.y) {
                commands.entity(entity).insert(WantsToMove { destination: dest });
            }
        }
        AIAction::Attack(target) => {
            commands.entity(entity).insert(WantsToAttack { target });
        }
        AIAction::Wait => {}
        AIAction::HealSelf => {}
        AIAction::SummonMinions => {}
        AIAction::SpecialAbility => {}
    }
}

fn boss_phase_transition(
    mut boss_query: Query<(&Health, &mut Boss, &mut Stats)>,
) {
    for (health, mut boss, mut stats) in boss_query.iter_mut() {
        let hp_percent = health.current as f32 / health.max as f32;

        let mut new_phase = 1;
        for (i, threshold) in boss.phase_thresholds.iter().enumerate() {
            if hp_percent <= *threshold {
                new_phase = i as i32 + 2;
            }
        }

        if new_phase != boss.phase {
            boss.phase = new_phase;
            apply_boss_phase_bonus(&mut stats, boss.phase);
        }
    }
}

fn apply_boss_phase_bonus(stats: &mut Stats, phase: i32) {
    let multiplier = 1.0 + (phase - 1) as f32 * 0.3;
    stats.attack = (stats.attack as f32 * multiplier) as i32;
    stats.speed = (stats.speed as f32 * (1.0 + (phase - 1) as f32 * 0.15)) as i32;
    stats.crit_rate += (phase - 1) as f32 * 0.05;
}
