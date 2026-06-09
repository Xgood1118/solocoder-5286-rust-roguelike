use bevy::prelude::*;
use crate::components::*;
use crate::bundles::*;
use crate::map::map::Rooms;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct MonsterTemplate {
    pub kind: EnemyKind,
    pub name: &'static str,
    pub name_zh: &'static str,
    pub glyph: char,
    pub color: Color,
    pub health: i32,
    pub stats: Stats,
    pub fov_range: i32,
    pub ai_type: AIType,
    pub xp_reward: i32,
    pub level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIType {
    Melee,
    Ranged,
    Caster,
    Healer,
    Summoner,
    Boss,
}

pub fn get_monster_templates() -> Vec<MonsterTemplate> {
    vec![
        MonsterTemplate {
            kind: EnemyKind::Slime,
            name: "Slime",
            name_zh: "史莱姆",
            glyph: 's',
            color: Color::rgb(0.0, 0.8, 0.0),
            health: 20,
            stats: Stats {
                attack: 5,
                defense: 2,
                speed: 5,
                crit_rate: 0.02,
                crit_damage: 1.5,
            },
            fov_range: 5,
            ai_type: AIType::Melee,
            xp_reward: 10,
            level: 1,
        },
        MonsterTemplate {
            kind: EnemyKind::Goblin,
            name: "Goblin",
            name_zh: "哥布林",
            glyph: 'g',
            color: Color::rgb(0.3, 0.7, 0.0),
            health: 30,
            stats: Stats {
                attack: 8,
                defense: 3,
                speed: 12,
                crit_rate: 0.10,
                crit_damage: 1.8,
            },
            fov_range: 6,
            ai_type: AIType::Melee,
            xp_reward: 20,
            level: 2,
        },
        MonsterTemplate {
            kind: EnemyKind::Skeleton,
            name: "Skeleton",
            name_zh: "骷髅兵",
            glyph: 'k',
            color: Color::WHITE,
            health: 35,
            stats: Stats {
                attack: 10,
                defense: 2,
                speed: 8,
                crit_rate: 0.05,
                crit_damage: 1.5,
            },
            fov_range: 7,
            ai_type: AIType::Melee,
            xp_reward: 25,
            level: 3,
        },
        MonsterTemplate {
            kind: EnemyKind::Orc,
            name: "Orc",
            name_zh: "兽人",
            glyph: 'o',
            color: Color::rgb(0.5, 0.4, 0.0),
            health: 60,
            stats: Stats {
                attack: 15,
                defense: 8,
                speed: 7,
                crit_rate: 0.08,
                crit_damage: 1.8,
            },
            fov_range: 8,
            ai_type: AIType::Melee,
            xp_reward: 50,
            level: 5,
        },
        MonsterTemplate {
            kind: EnemyKind::Troll,
            name: "Troll",
            name_zh: "巨魔",
            glyph: 'T',
            color: Color::rgb(0.6, 0.8, 0.2),
            health: 100,
            stats: Stats {
                attack: 20,
                defense: 10,
                speed: 5,
                crit_rate: 0.05,
                crit_damage: 2.0,
            },
            fov_range: 6,
            ai_type: AIType::Melee,
            xp_reward: 80,
            level: 7,
        },
        MonsterTemplate {
            kind: EnemyKind::Vampire,
            name: "Vampire",
            name_zh: "吸血鬼",
            glyph: 'v',
            color: Color::rgb(0.5, 0.0, 0.0),
            health: 80,
            stats: Stats {
                attack: 18,
                defense: 6,
                speed: 14,
                crit_rate: 0.20,
                crit_damage: 2.0,
            },
            fov_range: 9,
            ai_type: AIType::Melee,
            xp_reward: 100,
            level: 9,
        },
        MonsterTemplate {
            kind: EnemyKind::Dragon,
            name: "Dragon",
            name_zh: "巨龙",
            glyph: 'D',
            color: Color::RED,
            health: 200,
            stats: Stats {
                attack: 30,
                defense: 20,
                speed: 10,
                crit_rate: 0.15,
                crit_damage: 2.5,
            },
            fov_range: 12,
            ai_type: AIType::Ranged,
            xp_reward: 300,
            level: 15,
        },
        MonsterTemplate {
            kind: EnemyKind::BossGuardian,
            name: "Guardian Golem",
            name_zh: "守护者魔像",
            glyph: 'G',
            color: Color::YELLOW,
            health: 300,
            stats: Stats {
                attack: 25,
                defense: 25,
                speed: 6,
                crit_rate: 0.10,
                crit_damage: 2.0,
            },
            fov_range: 10,
            ai_type: AIType::Boss,
            xp_reward: 500,
            level: 10,
        },
        MonsterTemplate {
            kind: EnemyKind::BossNecromancer,
            name: "Necromancer Lord",
            name_zh: "亡灵法师",
            glyph: 'N',
            color: Color::PURPLE,
            health: 400,
            stats: Stats {
                attack: 35,
                defense: 15,
                speed: 10,
                crit_rate: 0.15,
                crit_damage: 2.2,
            },
            fov_range: 12,
            ai_type: AIType::Boss,
            xp_reward: 800,
            level: 15,
        },
        MonsterTemplate {
            kind: EnemyKind::BossDemonLord,
            name: "Demon Lord",
            name_zh: "恶魔领主",
            glyph: 'L',
            color: Color::rgb(0.8, 0.0, 0.0),
            health: 600,
            stats: Stats {
                attack: 45,
                defense: 20,
                speed: 12,
                crit_rate: 0.20,
                crit_damage: 2.5,
            },
            fov_range: 14,
            ai_type: AIType::Boss,
            xp_reward: 1200,
            level: 20,
        },
        MonsterTemplate {
            kind: EnemyKind::BossElderGod,
            name: "Elder God",
            name_zh: "远古之神",
            glyph: '*',
            color: Color::rgb(1.0, 0.84, 0.0),
            health: 1000,
            stats: Stats {
                attack: 60,
                defense: 30,
                speed: 15,
                crit_rate: 0.25,
                crit_damage: 3.0,
            },
            fov_range: 16,
            ai_type: AIType::Boss,
            xp_reward: 2000,
            level: 25,
        },
    ]
}

pub fn spawn_monsters_for_floor<R: Rng>(
    commands: &mut Commands,
    rooms: &Rooms,
    floor: i32,
    rng: &mut R,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let templates = get_monster_templates();

    let is_boss_floor = floor % 5 == 0;

    if is_boss_floor {
        if let Some(boss_room) = rooms.last() {
            let boss_template = get_boss_for_floor(floor, &templates);
            let boss_entity = spawn_monster(commands, boss_template, boss_room.center());
            entities.push(boss_entity);
        }
    } else {
        for room in rooms.iter().skip(1) {
            if rng.gen::<f32>() < 0.7 {
                let num_monsters = rng.gen_range(1..=3);
                let eligible_templates: Vec<&MonsterTemplate> = templates
                    .iter()
                    .filter(|t| !t.kind.is_boss() && t.level <= floor + 2 && t.level >= floor - 2)
                    .collect();

                if !eligible_templates.is_empty() {
                    for _ in 0..num_monsters {
                        let template = eligible_templates.choose(rng).unwrap();
                        let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
                        let y = rng.gen_range(room.y + 1..room.y + room.height - 1);
                        let entity = spawn_monster(commands, template, Position::new(x, y));
                        entities.push(entity);
                    }
                }
            }
        }
    }

    entities
}

fn get_boss_for_floor(floor: i32, templates: &[MonsterTemplate]) -> &MonsterTemplate {
    let bosses: Vec<&MonsterTemplate> = templates.iter().filter(|t| t.kind.is_boss()).collect();
    let boss_index = ((floor / 5 - 1) as usize).min(bosses.len().saturating_sub(1));
    bosses[boss_index]
}

pub fn spawn_monster(
    commands: &mut Commands,
    template: &MonsterTemplate,
    position: Position,
) -> Entity {
    let is_boss = template.kind.is_boss();

    if is_boss {
        commands.spawn(BossBundle {
            enemy: Enemy,
            boss: Boss {
                phase: 1,
                max_phases: 3,
                phase_thresholds: vec![0.75, 0.5, 0.25],
            },
            name: EntityName(template.name.to_string()),
            position,
            health: Health::new(template.health),
            stats: template.stats.clone(),
            fov: FieldOfView::new(template.fov_range),
            glyph: Glyph {
                symbol: template.glyph,
                color: template.color,
            },
            kind: template.kind.clone(),
            ai: AIControlled,
            blocks_movement: BlocksMovement,
            blocks_vision: BlocksVision,
            combatant: Combatant,
            turn_order: TurnOrder {
                initiative: template.stats.speed,
            },
        }).id()
    } else {
        commands.spawn(EnemyBundle {
            enemy: Enemy,
            name: EntityName(template.name.to_string()),
            position,
            health: Health::new(template.health),
            stats: template.stats.clone(),
            fov: FieldOfView::new(template.fov_range),
            glyph: Glyph {
                symbol: template.glyph,
                color: template.color,
            },
            kind: template.kind.clone(),
            ai: AIControlled,
            blocks_movement: BlocksMovement,
            blocks_vision: BlocksVision,
            combatant: Combatant,
            turn_order: TurnOrder {
                initiative: template.stats.speed,
            },
        }).id()
    }
}
