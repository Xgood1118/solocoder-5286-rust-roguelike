use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::resources::*;
use crate::components::*;
use crate::states::*;

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AchievementUnlockedEvent>()
            .add_systems(Update, check_achievements.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Event, Debug, Clone)]
pub struct AchievementUnlockedEvent {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub description: String,
    pub description_zh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub description: String,
    pub description_zh: String,
    pub category: AchievementCategory,
    pub rarity: AchievementRarity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementCategory {
    Combat,
    Exploration,
    Collection,
    Progression,
    Challenge,
    Secret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

pub fn get_all_achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_kill".to_string(),
            name: "First Blood".to_string(),
            name_zh: "首杀".to_string(),
            description: "Defeat your first enemy".to_string(),
            description_zh: "击败第一个敌人".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Common,
        },
        Achievement {
            id: "kill_100".to_string(),
            name: "Centurion".to_string(),
            name_zh: "百人斩".to_string(),
            description: "Defeat 100 enemies in a single run".to_string(),
            description_zh: "单局击败 100 个敌人".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Uncommon,
        },
        Achievement {
            id: "kill_1000".to_string(),
            name: "Slayer".to_string(),
            name_zh: "屠戮者".to_string(),
            description: "Defeat 1000 enemies total".to_string(),
            description_zh: "累计击败 1000 个敌人".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Rare,
        },
        Achievement {
            id: "boss_first".to_string(),
            name: "Boss Slayer".to_string(),
            name_zh: "Boss 杀手".to_string(),
            description: "Defeat your first boss".to_string(),
            description_zh: "击败第一个 Boss".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Uncommon,
        },
        Achievement {
            id: "boss_all".to_string(),
            name: "Conqueror".to_string(),
            name_zh: "征服者".to_string(),
            description: "Defeat all 4 bosses".to_string(),
            description_zh: "击败全部 4 个 Boss".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Epic,
        },
        Achievement {
            id: "floor_10".to_string(),
            name: "Deep Diver".to_string(),
            name_zh: "深渊探索者".to_string(),
            description: "Reach floor 10".to_string(),
            description_zh: "到达第 10 层".to_string(),
            category: AchievementCategory::Exploration,
            rarity: AchievementRarity::Rare,
        },
        Achievement {
            id: "floor_25".to_string(),
            name: "Dungeon Master".to_string(),
            name_zh: "地牢大师".to_string(),
            description: "Reach floor 25".to_string(),
            description_zh: "到达第 25 层".to_string(),
            category: AchievementCategory::Exploration,
            rarity: AchievementRarity::Epic,
        },
        Achievement {
            id: "no_damage_floor".to_string(),
            name: "Untouchable".to_string(),
            name_zh: "不可触碰".to_string(),
            description: "Clear a floor without taking damage".to_string(),
            description_zh: "无伤通关一层".to_string(),
            category: AchievementCategory::Challenge,
            rarity: AchievementRarity::Rare,
        },
        Achievement {
            id: "all_classes".to_string(),
            name: "Master of All".to_string(),
            name_zh: "全能大师".to_string(),
            description: "Win with all 6 classes".to_string(),
            description_zh: "使用全部 6 种职业通关".to_string(),
            category: AchievementCategory::Progression,
            rarity: AchievementRarity::Legendary,
        },
        Achievement {
            id: "legendary_item".to_string(),
            name: "Legendary Finder".to_string(),
            name_zh: "传说发现者".to_string(),
            description: "Find a legendary item".to_string(),
            description_zh: "发现一件传说物品".to_string(),
            category: AchievementCategory::Collection,
            rarity: AchievementRarity::Rare,
        },
        Achievement {
            id: "full_gear".to_string(),
            name: "Fully Equipped".to_string(),
            name_zh: "整装待发".to_string(),
            description: "Equip items in all slots".to_string(),
            description_zh: "装备所有槽位".to_string(),
            category: AchievementCategory::Collection,
            rarity: AchievementRarity::Uncommon,
        },
        Achievement {
            id: "gold_hoarder".to_string(),
            name: "Gold Hoarder".to_string(),
            name_zh: "黄金囤积者".to_string(),
            description: "Collect 1000 gold".to_string(),
            description_zh: "收集 1000 金币".to_string(),
            category: AchievementCategory::Collection,
            rarity: AchievementRarity::Uncommon,
        },
        Achievement {
            id: "first_death".to_string(),
            name: "Memento Mori".to_string(),
            name_zh: "死亡纪念".to_string(),
            description: "Die for the first time".to_string(),
            description_zh: "第一次死亡".to_string(),
            category: AchievementCategory::Progression,
            rarity: AchievementRarity::Common,
        },
        Achievement {
            id: "speedrun".to_string(),
            name: "Speedrunner".to_string(),
            name_zh: "速通达人".to_string(),
            description: "Beat the game in under 1000 turns".to_string(),
            description_zh: "1000 回合内通关".to_string(),
            category: AchievementCategory::Challenge,
            rarity: AchievementRarity::Epic,
        },
        Achievement {
            id: "perfect_run".to_string(),
            name: "Perfect Run".to_string(),
            name_zh: "完美通关".to_string(),
            description: "Beat the game without dying".to_string(),
            description_zh: "不死通关".to_string(),
            category: AchievementCategory::Challenge,
            rarity: AchievementRarity::Legendary,
        },
    ]
}

fn check_achievements(
    mut events: EventWriter<AchievementUnlockedEvent>,
    player_stats: Res<PlayerStats>,
    meta_progress: Res<MetaProgress>,
    game_seed: Res<GameSeed>,
) {
    let achievements = get_all_achievements();

    for achievement in achievements {
        if meta_progress.achievements.contains(&achievement.id) {
            continue;
        }

        check_single_achievement(
            &achievement,
            &player_stats,
            &meta_progress,
            &game_seed,
            &mut events,
        );
    }
}

fn check_single_achievement(
    achievement: &Achievement,
    player_stats: &PlayerStats,
    meta_progress: &MetaProgress,
    game_seed: &GameSeed,
    events: &mut EventWriter<AchievementUnlockedEvent>,
) {
    let unlocked = match achievement.id.as_str() {
        "first_kill" => player_stats.kills >= 1,
        "kill_100" => player_stats.kills >= 100,
        "kill_1000" => meta_progress.total_kills >= 1000,
        "floor_10" => game_seed.level >= 10,
        "floor_25" => game_seed.level >= 25,
        "first_death" => meta_progress.total_runs >= 1,
        _ => false,
    };

    if unlocked {
        events.send(AchievementUnlockedEvent {
            id: achievement.id.clone(),
            name: achievement.name.clone(),
            name_zh: achievement.name_zh.clone(),
            description: achievement.description.clone(),
            description_zh: achievement.description_zh.clone(),
        });
    }
}

pub fn unlock_achievement(meta: &mut MetaProgress, achievement_id: &str) {
    if !meta.achievements.iter().any(|a| a == achievement_id) {
        meta.achievements.push(achievement_id.to_string());
    }
}

pub fn is_achievement_unlocked(meta: &MetaProgress, achievement_id: &str) -> bool {
    meta.achievements.iter().any(|a| a == achievement_id)
}
