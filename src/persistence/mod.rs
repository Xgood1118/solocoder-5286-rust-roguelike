pub mod save;
pub mod replay;

use bevy::prelude::*;
use crate::states::*;
use crate::resources::*;

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MetaProgress>()
            .add_systems(OnEnter(GameState::GameOver), handle_permadeath)
            .add_systems(Update, auto_save.run_if(in_state(GameState::Playing)));
    }
}

fn auto_save(
    turn_counter: Res<TurnCounter>,
) {
}

fn handle_permadeath(
    mut meta_progress: ResMut<MetaProgress>,
    player_stats: Res<PlayerStats>,
    game_seed: Res<GameSeed>,
) {
    meta_progress.total_runs += 1;
    meta_progress.total_kills += player_stats.kills;

    let score = calculate_score(&player_stats, game_seed.level);
    if score > meta_progress.high_score {
        meta_progress.high_score = score;
    }

    if game_seed.level > meta_progress.best_floor {
        meta_progress.best_floor = game_seed.level;
    }
}

fn calculate_score(player_stats: &PlayerStats, floor: i32) -> u64 {
    let mut score = 0u64;
    score += player_stats.kills * 10;
    score += player_stats.damage_dealt / 10;
    score += player_stats.gold_found;
    score += floor as u64 * 100;
    score += player_stats.floors_cleared * 50;
    score
}
