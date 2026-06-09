use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub crit_rate: f32,
    pub crit_damage: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            attack: 10,
            defense: 5,
            speed: 10,
            crit_rate: 0.05,
            crit_damage: 2.0,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct EntityName(pub String);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Glyph {
    pub symbol: char,
    pub color: Color,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FieldOfView {
    pub range: i32,
    pub visible: Vec<Position>,
    pub dirty: bool,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            visible: Vec::new(),
            dirty: true,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BlocksMovement;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct BlocksVision;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Combatant;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyKind {
    Slime,
    Goblin,
    Skeleton,
    Orc,
    Troll,
    Vampire,
    Dragon,
    BossGuardian,
    BossNecromancer,
    BossDemonLord,
    BossElderGod,
}

impl EnemyKind {
    pub fn is_boss(&self) -> bool {
        matches!(
            self,
            EnemyKind::BossGuardian
                | EnemyKind::BossNecromancer
                | EnemyKind::BossDemonLord
                | EnemyKind::BossElderGod
        )
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Energy {
    pub current: i32,
    pub max: i32,
}

impl Energy {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn gain(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn spend(&mut self, amount: i32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TurnOrder {
    pub initiative: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WantsToMove {
    pub destination: Position,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WantsToAttack {
    pub target: Entity,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Position>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SufferDamage {
    pub amount: i32,
    pub is_crit: bool,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Heal {
    pub amount: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AIControlled;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Quest,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ItemBase {
    pub item_type: ItemType,
    pub base_name: String,
    pub rarity: Rarity,
    pub stats_bonus: Stats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Epic = 3,
    Legendary = 4,
}

impl Rarity {
    pub fn color(&self) -> Color {
        match self {
            Rarity::Common => Color::WHITE,
            Rarity::Uncommon => Color::GREEN,
            Rarity::Rare => Color::BLUE,
            Rarity::Epic => Color::PURPLE,
            Rarity::Legendary => Color::ORANGE,
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Affix {
    pub name: String,
    pub stats_bonus: Stats,
    pub tier: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Equipped;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentSlot {
    pub slot: EquipmentSlotKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlotKind {
    Weapon,
    Offhand,
    Head,
    Chest,
    Hands,
    Feet,
    Accessory1,
    Accessory2,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct InventoryPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CarriedBy(pub Entity);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct OnGround;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DungeonLevel(pub i32);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Explored;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Visible;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct DownStairs;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct UpStairs;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Trap {
    pub triggered: bool,
    pub damage: i32,
    pub effect: TrapEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrapEffect {
    Damage,
    Poison,
    Slow,
    Teleport,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect: StatusEffectKind,
    pub duration: i32,
    pub magnitude: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectKind {
    Poison,
    Burning,
    Frozen,
    Stunned,
    Weakened,
    Enraged,
    Shielded,
    Invisible,
    Hasted,
    Slowed,
    Regeneration,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Boss {
    pub phase: i32,
    pub max_phases: i32,
    pub phase_thresholds: Vec<f32>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerClass {
    pub class: CharacterClass,
    pub level: i32,
    pub experience: i32,
    pub experience_to_next: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterClass {
    Warrior,
    Rogue,
    Mage,
    Priest,
    Ranger,
    Warlock,
}

impl CharacterClass {
    pub fn name(&self) -> &'static str {
        match self {
            CharacterClass::Warrior => "Warrior",
            CharacterClass::Rogue => "Rogue",
            CharacterClass::Mage => "Mage",
            CharacterClass::Priest => "Priest",
            CharacterClass::Ranger => "Ranger",
            CharacterClass::Warlock => "Warlock",
        }
    }

    pub fn name_zh(&self) -> &'static str {
        match self {
            CharacterClass::Warrior => "战士",
            CharacterClass::Rogue => "盗贼",
            CharacterClass::Mage => "法师",
            CharacterClass::Priest => "牧师",
            CharacterClass::Ranger => "游侠",
            CharacterClass::Warlock => "术士",
        }
    }

    pub fn base_stats(&self) -> Stats {
        match self {
            CharacterClass::Warrior => Stats {
                attack: 12,
                defense: 8,
                speed: 8,
                crit_rate: 0.05,
                crit_damage: 2.0,
            },
            CharacterClass::Rogue => Stats {
                attack: 10,
                defense: 4,
                speed: 14,
                crit_rate: 0.25,
                crit_damage: 2.5,
            },
            CharacterClass::Mage => Stats {
                attack: 15,
                defense: 3,
                speed: 10,
                crit_rate: 0.10,
                crit_damage: 2.0,
            },
            CharacterClass::Priest => Stats {
                attack: 8,
                defense: 6,
                speed: 10,
                crit_rate: 0.05,
                crit_damage: 1.5,
            },
            CharacterClass::Ranger => Stats {
                attack: 11,
                defense: 5,
                speed: 12,
                crit_rate: 0.15,
                crit_damage: 2.2,
            },
            CharacterClass::Warlock => Stats {
                attack: 14,
                defense: 4,
                speed: 9,
                crit_rate: 0.08,
                crit_damage: 2.3,
            },
        }
    }

    pub fn base_health(&self) -> i32 {
        match self {
            CharacterClass::Warrior => 120,
            CharacterClass::Rogue => 80,
            CharacterClass::Mage => 70,
            CharacterClass::Priest => 90,
            CharacterClass::Ranger => 95,
            CharacterClass::Warlock => 75,
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            CharacterClass::Warrior => '@',
            CharacterClass::Rogue => '@',
            CharacterClass::Mage => '@',
            CharacterClass::Priest => '@',
            CharacterClass::Ranger => '@',
            CharacterClass::Warlock => '@',
        }
    }

    pub fn starting_weapon(&self) -> &'static str {
        match self {
            CharacterClass::Warrior => "iron_sword",
            CharacterClass::Rogue => "iron_dagger",
            CharacterClass::Mage => "apprentice_staff",
            CharacterClass::Priest => "wooden_mace",
            CharacterClass::Ranger => "hunting_bow",
            CharacterClass::Warlock => "apprentice_wand",
        }
    }
}
