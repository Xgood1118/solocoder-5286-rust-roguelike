pub mod damage;
pub mod status_effects;

use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::resources::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CombatState::default())
            .add_systems(
                Update,
                (
                    handle_attack_requests,
                    apply_damage,
                    apply_healing,
                    check_deaths,
                    process_status_effects,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn handle_attack_requests(
    mut commands: Commands,
    attacker_query: Query<(&Stats, Option<&Player>), With<Combatant>>,
    attack_events: Query<(Entity, &WantsToAttack)>,
    mut rng: ResMut<GameRng>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let mut attacks_to_process = Vec::new();

    for (attacker_entity, wants_attack) in attack_events.iter() {
        if let Ok((attacker_stats, is_player_attacker)) = attacker_query.get(attacker_entity) {
            attacks_to_process.push((
                attacker_entity,
                wants_attack.target,
                attacker_stats.clone(),
                is_player_attacker.is_some(),
            ));
        }
        commands.entity(attacker_entity).remove::<WantsToAttack>();
    }

    for (_attacker_entity, target_entity, attacker_stats, is_player_attacker) in attacks_to_process {
        if let Ok((target_stats, _)) = attacker_query.get(target_entity) {
            let damage_info = damage::calculate_damage(
                &attacker_stats,
                target_stats,
                &mut rng.rng,
            );

            commands.entity(target_entity).insert(SufferDamage {
                amount: damage_info.total,
                is_crit: damage_info.is_crit,
            });

            if is_player_attacker {
                player_stats.damage_dealt += damage_info.total as u64;
            }
        }
    }
}

fn apply_damage(
    mut commands: Commands,
    mut health_query: Query<(Entity, &mut Health, Option<&Player>)>,
    damage_query: Query<&SufferDamage>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let mut to_remove = Vec::new();

    for (entity, mut health, is_player) in health_query.iter_mut() {
        if let Ok(suffer) = damage_query.get(entity) {
            health.take_damage(suffer.amount);
            to_remove.push(entity);

            if is_player.is_some() {
                player_stats.damage_taken += suffer.amount as u64;
            }
        }
    }

    for entity in to_remove {
        commands.entity(entity).remove::<SufferDamage>();
    }
}

fn apply_healing(
    mut commands: Commands,
    mut health_query: Query<(Entity, &mut Health)>,
    heal_query: Query<&Heal>,
) {
    let mut to_remove = Vec::new();

    for (entity, mut health) in health_query.iter_mut() {
        if let Ok(heal) = heal_query.get(entity) {
            health.heal(heal.amount);
            to_remove.push(entity);
        }
    }

    for entity in to_remove {
        commands.entity(entity).remove::<Heal>();
    }
}

fn check_deaths(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>, Option<&Enemy>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for (entity, health, is_player, is_enemy) in query.iter() {
        if health.is_dead() {
            if is_player.is_some() {
                next_state.set(GameState::GameOver);
            } else if is_enemy.is_some() {
                commands.entity(entity).despawn();
                player_stats.kills += 1;
            }
        }
    }
}

fn process_status_effects(
    mut commands: Commands,
    mut health_query: Query<&mut Health>,
    mut status_query: Query<(Entity, &mut StatusEffect)>,
    _turn_counter: Res<TurnCounter>,
) {
    let mut to_remove = Vec::new();

    for (entity, mut status) in status_query.iter_mut() {
        status.duration -= 1;

        match status.effect {
            StatusEffectKind::Poison | StatusEffectKind::Burning => {
                if let Ok(mut health) = health_query.get_mut(entity) {
                    health.take_damage(status.magnitude);
                }
            }
            StatusEffectKind::Regeneration => {
                if let Ok(mut health) = health_query.get_mut(entity) {
                    health.heal(status.magnitude);
                }
            }
            _ => {}
        }

        if status.duration <= 0 {
            to_remove.push(entity);
        }
    }

    for entity in to_remove {
        commands.entity(entity).remove::<StatusEffect>();
    }
}
