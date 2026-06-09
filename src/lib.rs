pub mod components;
pub mod bundles;
pub mod resources;
pub mod states;
pub mod map;
pub mod combat;
pub mod entities;
pub mod ai;
pub mod inventory;
pub mod persistence;
pub mod render;
pub mod achievements;
pub mod i18n;

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::inventory::{Inventory, Equipment};

#[derive(Resource, Debug, Clone, Default)]
struct BlockerPositions {
    positions: Vec<Position>,
}

pub struct RoguelikePlugin;

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::GeneratingMap)
            .init_resource::<GameRng>()
            .init_resource::<GameSeed>()
            .init_resource::<TurnCounter>()
            .init_resource::<PlayerStats>()
            .init_resource::<Settings>()
            .init_resource::<ReplayData>()
            .insert_resource(Inventory::new(8, 6))
            .init_resource::<Equipment>()
            .init_resource::<BlockerPositions>()
            .add_systems(
                Update,
                (
                    player_input,
                    update_blocker_positions,
                    handle_movement.after(update_blocker_positions),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_plugins((
                map::MapPlugin,
                combat::CombatPlugin,
                entities::EntitiesPlugin,
                ai::AiPlugin,
                inventory::InventoryPlugin,
                persistence::PersistencePlugin,
                render::RenderPlugin,
                achievements::AchievementPlugin,
                i18n::I18nPlugin,
            ));
    }
}

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Energy), With<Player>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    mut turn_counter: ResMut<TurnCounter>,
) {
    if let Ok((player_entity, pos, mut energy)) = player_query.get_single_mut() {
        if energy.current < 10 {
            return;
        }

        let mut delta = (0, 0);

        if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
            delta = (0, -1);
        } else if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
            delta = (0, 1);
        } else if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            delta = (-1, 0);
        } else if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
            delta = (1, 0);
        }

        if delta != (0, 0) {
            let new_pos = Position::new(pos.x + delta.0, pos.y + delta.1);

            let mut attacked = false;
            for (enemy_entity, enemy_pos) in enemy_query.iter() {
                if enemy_pos == &new_pos {
                    commands.entity(player_entity).insert(WantsToAttack { target: enemy_entity });
                    attacked = true;
                    break;
                }
            }

            if !attacked {
                commands.entity(player_entity).insert(WantsToMove { destination: new_pos });
            }

            energy.current -= 10;
            turn_counter.turn += 1;

            if energy.current <= 0 {
                energy.current = 100;
            }
        }
    }
}

fn update_blocker_positions(
    mut blocker_positions: ResMut<BlockerPositions>,
    blocker_query: Query<&Position, With<BlocksMovement>>,
) {
    blocker_positions.positions.clear();
    for pos in blocker_query.iter() {
        blocker_positions.positions.push(*pos);
    }
}

fn handle_movement(
    mut commands: Commands,
    mut move_query: Query<(Entity, &mut Position, &WantsToMove)>,
    map: Res<crate::map::map::GameMap>,
    blocker_positions: Res<BlockerPositions>,
) {
    let mut to_remove = Vec::new();

    for (entity, mut pos, wants_move) in move_query.iter_mut() {
        let dest = &wants_move.destination;

        if !map.in_bounds(dest.x, dest.y) {
            to_remove.push(entity);
            continue;
        }

        if map.is_occupied(dest.x, dest.y) {
            to_remove.push(entity);
            continue;
        }

        let blocked_by_entity = blocker_positions
            .positions
            .iter()
            .any(|p| p == dest && p != &*pos);

        if blocked_by_entity {
            to_remove.push(entity);
            continue;
        }

        pos.x = dest.x;
        pos.y = dest.y;
        to_remove.push(entity);
    }

    for entity in to_remove {
        commands.entity(entity).remove::<WantsToMove>();
    }
}
