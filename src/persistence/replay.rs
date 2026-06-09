use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::resources::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Replay {
    pub seed: u64,
    pub player_class: String,
    pub duration_turns: u64,
    pub inputs: Vec<ReplayFrame>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplayFrame {
    pub turn: u64,
    pub action: PlayerAction,
    pub player_pos: (i32, i32),
    pub player_health: i32,
}

pub fn record_input(
    replay_data: &mut ReplayData,
    turn: u64,
    action: PlayerAction,
) {
    replay_data.inputs.push(ReplayInput { turn, action });
}

pub fn save_replay(replay: &Replay, name: &str) -> Result<(), String> {
    let mut path = get_replay_directory();
    path.push(format!("{}.replay", name));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let data = bincode::serialize(replay).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_replay(name: &str) -> Result<Replay, String> {
    let mut path = get_replay_directory();
    path.push(format!("{}.replay", name));

    let data = fs::read(path).map_err(|e| e.to_string())?;
    let replay: Replay = bincode::deserialize(&data).map_err(|e| e.to_string())?;

    Ok(replay)
}

fn get_replay_directory() -> PathBuf {
    let mut path = PathBuf::from("data");
    path.push("replays");
    path
}

pub fn list_replays() -> Result<Vec<String>, String> {
    let path = get_replay_directory();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut replays = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".replay") {
                replays.push(filename.trim_end_matches(".replay").to_string());
            }
        }
    }

    Ok(replays)
}
