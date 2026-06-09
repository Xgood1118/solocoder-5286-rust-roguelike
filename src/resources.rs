use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone)]
pub struct GameRng {
    pub rng: ChaCha8Rng,
}

impl GameRng {
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameSeed {
    pub seed: u64,
    pub level: i32,
}

impl Default for GameSeed {
    fn default() -> Self {
        Self {
            seed: 12345,
            level: 1,
        }
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct TurnCounter {
    pub turn: u64,
}

impl Default for TurnCounter {
    fn default() -> Self {
        Self { turn: 0 }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MapConfig {
    pub width: i32,
    pub height: i32,
    pub min_room_size: i32,
    pub max_room_size: i32,
    pub max_rooms: i32,
    pub bsp_depth: i32,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            width: 80,
            height: 50,
            min_room_size: 6,
            max_room_size: 14,
            max_rooms: 30,
            bsp_depth: 6,
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct PlayerStats {
    pub kills: u64,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub items_picked_up: u64,
    pub gold_found: u64,
    pub floors_cleared: u64,
    pub damage_avoided: u64,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgress {
    pub high_score: u64,
    pub total_runs: u64,
    pub total_kills: u64,
    pub best_floor: i32,
    pub unlocked_classes: Vec<String>,
    pub achievements: Vec<String>,
    pub play_time_seconds: u64,
}

impl Default for MetaProgress {
    fn default() -> Self {
        Self {
            high_score: 0,
            total_runs: 0,
            total_kills: 0,
            best_floor: 0,
            unlocked_classes: vec![
                "Warrior".to_string(),
                "Rogue".to_string(),
                "Mage".to_string(),
            ],
            achievements: Vec::new(),
            play_time_seconds: 0,
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct Settings {
    pub render_mode: RenderModeSetting,
    pub language: Language,
    pub sound_volume: f32,
    pub music_volume: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderModeSetting {
    Ascii,
    Tile,
}

impl Default for RenderModeSetting {
    fn default() -> Self {
        RenderModeSetting::Ascii
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Chinese
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct ReplayData {
    pub seed: u64,
    pub inputs: Vec<ReplayInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayInput {
    pub turn: u64,
    pub action: PlayerAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Wait,
    Interact,
    UseItem(usize),
    EquipItem(usize),
    DropItem(usize),
}
