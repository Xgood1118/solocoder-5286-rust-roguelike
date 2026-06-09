use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::resources::*;

const SAVE_DIRECTORY: &str = "saves";
const SAVE_FILE: &str = "game_save.bin";
const META_PROGRESS_FILE: &str = "meta_progress.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSaveData {
    pub seed: u64,
    pub level: i32,
    pub turn: u64,
    pub player_position: (i32, i32),
    pub player_health: i32,
    pub player_max_health: i32,
}

pub fn get_save_directory() -> PathBuf {
    let mut path = PathBuf::from("data");
    path.push(SAVE_DIRECTORY);
    path
}

pub fn get_save_path() -> PathBuf {
    let mut path = get_save_directory();
    path.push(SAVE_FILE);
    path
}

pub fn get_meta_progress_path() -> PathBuf {
    let mut path = get_save_directory();
    path.push(META_PROGRESS_FILE);
    path
}

pub fn save_meta_progress(meta: &MetaProgress) -> Result<(), String> {
    let path = get_meta_progress_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_meta_progress() -> Result<MetaProgress, String> {
    let path = get_meta_progress_path();
    if !path.exists() {
        return Ok(MetaProgress::default());
    }

    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let meta: MetaProgress = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(meta)
}

pub fn delete_save_file() -> Result<(), String> {
    let path = get_save_path();
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn has_save_file() -> bool {
    get_save_path().exists()
}
