use bevy::prelude::*;
use crate::components::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponTemplate {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub base_attack: i32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub speed: i32,
    pub glyph: char,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Staff,
    Dagger,
    Mace,
    Polearm,
}

pub fn get_weapon_templates() -> Vec<WeaponTemplate> {
    vec![
        WeaponTemplate {
            id: "iron_sword".to_string(),
            name: "Iron Sword".to_string(),
            name_zh: "铁剑".to_string(),
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Common,
            base_attack: 10,
            crit_rate: 0.05,
            crit_damage: 2.0,
            speed: 10,
            glyph: '/',
            color: Color::WHITE,
        },
        WeaponTemplate {
            id: "steel_sword".to_string(),
            name: "Steel Sword".to_string(),
            name_zh: "钢剑".to_string(),
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Uncommon,
            base_attack: 18,
            crit_rate: 0.07,
            crit_damage: 2.0,
            speed: 10,
            glyph: '/',
            color: Color::GREEN,
        },
        WeaponTemplate {
            id: "rune_blade".to_string(),
            name: "Rune Blade".to_string(),
            name_zh: "符文之刃".to_string(),
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Rare,
            base_attack: 28,
            crit_rate: 0.10,
            crit_damage: 2.2,
            speed: 10,
            glyph: '/',
            color: Color::BLUE,
        },
        WeaponTemplate {
            id: "dragon_slayer".to_string(),
            name: "Dragon Slayer".to_string(),
            name_zh: "屠龙剑".to_string(),
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Epic,
            base_attack: 40,
            crit_rate: 0.12,
            crit_damage: 2.5,
            speed: 9,
            glyph: '/',
            color: Color::PURPLE,
        },
        WeaponTemplate {
            id: "excalibur".to_string(),
            name: "Excalibur".to_string(),
            name_zh: "圣剑".to_string(),
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Legendary,
            base_attack: 60,
            crit_rate: 0.15,
            crit_damage: 3.0,
            speed: 12,
            glyph: '/',
            color: Color::GOLD,
        },
        WeaponTemplate {
            id: "iron_dagger".to_string(),
            name: "Iron Dagger".to_string(),
            name_zh: "铁匕首".to_string(),
            weapon_type: WeaponType::Dagger,
            rarity: Rarity::Common,
            base_attack: 7,
            crit_rate: 0.15,
            crit_damage: 2.0,
            speed: 15,
            glyph: '-',
            color: Color::WHITE,
        },
        WeaponTemplate {
            id: "apprentice_staff".to_string(),
            name: "Apprentice Staff".to_string(),
            name_zh: "学徒法杖".to_string(),
            weapon_type: WeaponType::Staff,
            rarity: Rarity::Common,
            base_attack: 12,
            crit_rate: 0.03,
            crit_damage: 2.0,
            speed: 8,
            glyph: '?',
            color: Color::WHITE,
        },
        WeaponTemplate {
            id: "wooden_mace".to_string(),
            name: "Wooden Mace".to_string(),
            name_zh: "木锤".to_string(),
            weapon_type: WeaponType::Mace,
            rarity: Rarity::Common,
            base_attack: 9,
            crit_rate: 0.03,
            crit_damage: 1.8,
            speed: 8,
            glyph: '!',
            color: Color::WHITE,
        },
        WeaponTemplate {
            id: "hunting_bow".to_string(),
            name: "Hunting Bow".to_string(),
            name_zh: "猎弓".to_string(),
            weapon_type: WeaponType::Bow,
            rarity: Rarity::Common,
            base_attack: 9,
            crit_rate: 0.08,
            crit_damage: 2.0,
            speed: 11,
            glyph: ')',
            color: Color::WHITE,
        },
        WeaponTemplate {
            id: "spear".to_string(),
            name: "Spear".to_string(),
            name_zh: "长矛".to_string(),
            weapon_type: WeaponType::Polearm,
            rarity: Rarity::Common,
            base_attack: 11,
            crit_rate: 0.05,
            crit_damage: 2.0,
            speed: 10,
            glyph: '|',
            color: Color::WHITE,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorTemplate {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub armor_type: ArmorType,
    pub slot: EquipmentSlotKind,
    pub rarity: Rarity,
    pub base_defense: i32,
    pub speed: i32,
    pub glyph: char,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

pub fn get_armor_templates() -> Vec<ArmorTemplate> {
    vec![
        ArmorTemplate {
            id: "leather_armor".to_string(),
            name: "Leather Armor".to_string(),
            name_zh: "皮甲".to_string(),
            armor_type: ArmorType::Light,
            slot: EquipmentSlotKind::Chest,
            rarity: Rarity::Common,
            base_defense: 5,
            speed: 2,
            glyph: '[',
            color: Color::WHITE,
        },
        ArmorTemplate {
            id: "chain_mail".to_string(),
            name: "Chain Mail".to_string(),
            name_zh: "锁子甲".to_string(),
            armor_type: ArmorType::Medium,
            slot: EquipmentSlotKind::Chest,
            rarity: Rarity::Uncommon,
            base_defense: 10,
            speed: 0,
            glyph: '[',
            color: Color::GREEN,
        },
        ArmorTemplate {
            id: "plate_armor".to_string(),
            name: "Plate Armor".to_string(),
            name_zh: "板甲".to_string(),
            armor_type: ArmorType::Heavy,
            slot: EquipmentSlotKind::Chest,
            rarity: Rarity::Rare,
            base_defense: 18,
            speed: -2,
            glyph: '[',
            color: Color::BLUE,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTemplate {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub consumable_type: ConsumableType,
    pub rarity: Rarity,
    pub value: i32,
    pub glyph: char,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion,
    ManaPotion,
    Scroll,
    Food,
    Bomb,
}

pub fn get_consumable_templates() -> Vec<ConsumableTemplate> {
    vec![
        ConsumableTemplate {
            id: "health_potion_small".to_string(),
            name: "Small Health Potion".to_string(),
            name_zh: "小型治疗药水".to_string(),
            consumable_type: ConsumableType::HealthPotion,
            rarity: Rarity::Common,
            value: 25,
            glyph: '!',
            color: Color::RED,
        },
        ConsumableTemplate {
            id: "health_potion_medium".to_string(),
            name: "Medium Health Potion".to_string(),
            name_zh: "中型治疗药水".to_string(),
            consumable_type: ConsumableType::HealthPotion,
            rarity: Rarity::Uncommon,
            value: 50,
            glyph: '!',
            color: Color::RED,
        },
        ConsumableTemplate {
            id: "health_potion_large".to_string(),
            name: "Large Health Potion".to_string(),
            name_zh: "大型治疗药水".to_string(),
            consumable_type: ConsumableType::HealthPotion,
            rarity: Rarity::Rare,
            value: 100,
            glyph: '!',
            color: Color::RED,
        },
    ]
}
