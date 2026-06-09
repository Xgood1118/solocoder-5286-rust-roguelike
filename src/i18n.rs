use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<I18n>();
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct I18n {
    pub current_language: Language,
    pub translations: HashMap<String, String>,
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

impl I18n {
    pub fn new(language: Language) -> Self {
        let mut i18n = Self {
            current_language: language,
            translations: HashMap::new(),
        };
        i18n.load_translations();
        i18n
    }

    pub fn load_translations(&mut self) {
        self.translations.clear();

        let translations = match self.current_language {
            Language::Chinese => chinese_translations(),
            Language::English => english_translations(),
        };

        for (key, value) in translations {
            self.translations.insert(key.to_string(), value.to_string());
        }
    }

    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
        self.load_translations();
    }
}

fn chinese_translations() -> Vec<(&'static str, &'static str)> {
    vec![
        ("game_title", "Roguelike 地牢"),
        ("main_menu", "主菜单"),
        ("new_game", "新游戏"),
        ("continue", "继续游戏"),
        ("settings", "设置"),
        ("quit", "退出"),
        ("character_select", "选择职业"),
        ("seed_input", "输入种子"),
        ("inventory", "背包"),
        ("equipment", "装备"),
        ("stats", "属性"),
        ("health", "生命"),
        ("attack", "攻击"),
        ("defense", "防御"),
        ("speed", "速度"),
        ("crit_rate", "暴击率"),
        ("crit_damage", "暴击伤害"),
        ("floor", "层"),
        ("turn", "回合"),
        ("game_over", "游戏结束"),
        ("victory", "胜利"),
        ("achievements", "成就"),
        ("you_died", "你死了"),
        ("press_any_key", "按任意键继续"),
        ("warrior", "战士"),
        ("rogue", "盗贼"),
        ("mage", "法师"),
        ("priest", "牧师"),
        ("ranger", "游侠"),
        ("warlock", "术士"),
        ("weapon", "武器"),
        ("armor", "防具"),
        ("consumable", "消耗品"),
        ("material", "材料"),
        ("quest", "任务"),
        ("common", "普通"),
        ("uncommon", "优秀"),
        ("rare", "稀有"),
        ("epic", "史诗"),
        ("legendary", "传说"),
        ("move_up", "上移"),
        ("move_down", "下移"),
        ("move_left", "左移"),
        ("move_right", "右移"),
        ("wait", "等待"),
        ("interact", "交互"),
        ("use_item", "使用物品"),
        ("equip_item", "装备物品"),
        ("drop_item", "丢弃物品"),
        ("level_up", "升级了！"),
        ("damage", "伤害"),
        ("critical_hit", "暴击！"),
        ("heal", "治疗"),
        ("poisoned", "中毒"),
        ("burning", "燃烧"),
        ("frozen", "冰冻"),
        ("stunned", "眩晕"),
        ("weakened", "虚弱"),
        ("enraged", "狂暴"),
        ("shielded", "护盾"),
        ("invisible", "隐身"),
        ("hasted", "加速"),
        ("slowed", "减速"),
        ("regeneration", "再生"),
        ("achievement_unlocked", "成就解锁！"),
        ("render_mode", "渲染模式"),
        ("ascii_mode", "ASCII 模式"),
        ("tile_mode", "Tile 模式"),
        ("language", "语言"),
        ("sound_volume", "音量"),
        ("music_volume", "音乐"),
    ]
}

fn english_translations() -> Vec<(&'static str, &'static str)> {
    vec![
        ("game_title", "Roguelike Dungeon"),
        ("main_menu", "Main Menu"),
        ("new_game", "New Game"),
        ("continue", "Continue"),
        ("settings", "Settings"),
        ("quit", "Quit"),
        ("character_select", "Select Class"),
        ("seed_input", "Enter Seed"),
        ("inventory", "Inventory"),
        ("equipment", "Equipment"),
        ("stats", "Stats"),
        ("health", "Health"),
        ("attack", "Attack"),
        ("defense", "Defense"),
        ("speed", "Speed"),
        ("crit_rate", "Crit Rate"),
        ("crit_damage", "Crit Damage"),
        ("floor", "Floor"),
        ("turn", "Turn"),
        ("game_over", "Game Over"),
        ("victory", "Victory"),
        ("achievements", "Achievements"),
        ("you_died", "You Died"),
        ("press_any_key", "Press any key to continue"),
        ("warrior", "Warrior"),
        ("rogue", "Rogue"),
        ("mage", "Mage"),
        ("priest", "Priest"),
        ("ranger", "Ranger"),
        ("warlock", "Warlock"),
        ("weapon", "Weapon"),
        ("armor", "Armor"),
        ("consumable", "Consumable"),
        ("material", "Material"),
        ("quest", "Quest"),
        ("common", "Common"),
        ("uncommon", "Uncommon"),
        ("rare", "Rare"),
        ("epic", "Epic"),
        ("legendary", "Legendary"),
        ("move_up", "Move Up"),
        ("move_down", "Move Down"),
        ("move_left", "Move Left"),
        ("move_right", "Move Right"),
        ("wait", "Wait"),
        ("interact", "Interact"),
        ("use_item", "Use Item"),
        ("equip_item", "Equip Item"),
        ("drop_item", "Drop Item"),
        ("level_up", "Level Up!"),
        ("damage", "Damage"),
        ("critical_hit", "Critical Hit!"),
        ("heal", "Heal"),
        ("poisoned", "Poisoned"),
        ("burning", "Burning"),
        ("frozen", "Frozen"),
        ("stunned", "Stunned"),
        ("weakened", "Weakened"),
        ("enraged", "Enraged"),
        ("shielded", "Shielded"),
        ("invisible", "Invisible"),
        ("hasted", "Hasted"),
        ("slowed", "Slowed"),
        ("regeneration", "Regeneration"),
        ("achievement_unlocked", "Achievement Unlocked!"),
        ("render_mode", "Render Mode"),
        ("ascii_mode", "ASCII Mode"),
        ("tile_mode", "Tile Mode"),
        ("language", "Language"),
        ("sound_volume", "Sound Volume"),
        ("music_volume", "Music Volume"),
    ]
}
