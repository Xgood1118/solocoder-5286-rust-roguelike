use crate::components::*;

pub fn get_boss_abilities(kind: &EnemyKind, phase: i32) -> Vec<BossAbility> {
    match kind {
        EnemyKind::BossGuardian => guardian_abilities(phase),
        EnemyKind::BossNecromancer => necromancer_abilities(phase),
        EnemyKind::BossDemonLord => demon_lord_abilities(phase),
        EnemyKind::BossElderGod => elder_god_abilities(phase),
        _ => Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub struct BossAbility {
    pub name: String,
    pub name_zh: String,
    pub damage: i32,
    pub cooldown: i32,
    pub current_cooldown: i32,
    pub effect: Option<StatusEffectKind>,
    pub effect_duration: i32,
    pub ability_type: BossAbilityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossAbilityType {
    MeleeAttack,
    RangedAttack,
    AreaAttack,
    Buff,
    Debuff,
    Summon,
    Heal,
}

fn guardian_abilities(phase: i32) -> Vec<BossAbility> {
    let mut abilities = vec![
        BossAbility {
            name: "Ground Smash".to_string(),
            name_zh: "大地粉碎".to_string(),
            damage: 20 + phase * 5,
            cooldown: 3,
            current_cooldown: 0,
            effect: Some(StatusEffectKind::Stunned),
            effect_duration: 1,
            ability_type: BossAbilityType::AreaAttack,
        },
        BossAbility {
            name: "Stone Form".to_string(),
            name_zh: "岩石形态".to_string(),
            damage: 0,
            cooldown: 5,
            current_cooldown: 2,
            effect: Some(StatusEffectKind::Shielded),
            effect_duration: 3,
            ability_type: BossAbilityType::Buff,
        },
    ];

    if phase >= 2 {
        abilities.push(BossAbility {
            name: "Rage".to_string(),
            name_zh: "狂怒".to_string(),
            damage: 0,
            cooldown: 6,
            current_cooldown: 3,
            effect: Some(StatusEffectKind::Enraged),
            effect_duration: 4,
            ability_type: BossAbilityType::Buff,
        });
    }

    if phase >= 3 {
        abilities.push(BossAbility {
            name: "Earthquake".to_string(),
            name_zh: "地震".to_string(),
            damage: 40,
            cooldown: 8,
            current_cooldown: 5,
            effect: Some(StatusEffectKind::Stunned),
            effect_duration: 2,
            ability_type: BossAbilityType::AreaAttack,
        });
    }

    abilities
}

fn necromancer_abilities(phase: i32) -> Vec<BossAbility> {
    let mut abilities = vec![
        BossAbility {
            name: "Death Bolt".to_string(),
            name_zh: "死亡箭".to_string(),
            damage: 25 + phase * 8,
            cooldown: 2,
            current_cooldown: 0,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::RangedAttack,
        },
        BossAbility {
            name: "Raise Skeleton".to_string(),
            name_zh: "召唤骷髅".to_string(),
            damage: 0,
            cooldown: 5,
            current_cooldown: 2,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::Summon,
        },
    ];

    if phase >= 2 {
        abilities.push(BossAbility {
            name: "Curse".to_string(),
            name_zh: "诅咒".to_string(),
            damage: 0,
            cooldown: 6,
            current_cooldown: 3,
            effect: Some(StatusEffectKind::Weakened),
            effect_duration: 5,
            ability_type: BossAbilityType::Debuff,
        });
    }

    if phase >= 3 {
        abilities.push(BossAbility {
            name: "Death Nova".to_string(),
            name_zh: "死亡新星".to_string(),
            damage: 50,
            cooldown: 10,
            current_cooldown: 6,
            effect: Some(StatusEffectKind::Poison),
            effect_duration: 5,
            ability_type: BossAbilityType::AreaAttack,
        });
    }

    abilities
}

fn demon_lord_abilities(phase: i32) -> Vec<BossAbility> {
    let mut abilities = vec![
        BossAbility {
            name: "Hellfire".to_string(),
            name_zh: "地狱火".to_string(),
            damage: 30 + phase * 10,
            cooldown: 2,
            current_cooldown: 0,
            effect: Some(StatusEffectKind::Burning),
            effect_duration: 3,
            ability_type: BossAbilityType::RangedAttack,
        },
        BossAbility {
            name: "Demon Charge".to_string(),
            name_zh: "恶魔冲锋".to_string(),
            damage: 40,
            cooldown: 4,
            current_cooldown: 2,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::MeleeAttack,
        },
    ];

    if phase >= 2 {
        abilities.push(BossAbility {
            name: "Summon Imp".to_string(),
            name_zh: "召唤小鬼".to_string(),
            damage: 0,
            cooldown: 6,
            current_cooldown: 3,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::Summon,
        });
    }

    if phase >= 3 {
        abilities.push(BossAbility {
            name: "Apocalypse".to_string(),
            name_zh: "天启".to_string(),
            damage: 80,
            cooldown: 12,
            current_cooldown: 8,
            effect: Some(StatusEffectKind::Burning),
            effect_duration: 5,
            ability_type: BossAbilityType::AreaAttack,
        });
    }

    abilities
}

fn elder_god_abilities(phase: i32) -> Vec<BossAbility> {
    let mut abilities = vec![
        BossAbility {
            name: "Cosmic Beam".to_string(),
            name_zh: "宇宙射线".to_string(),
            damage: 50 + phase * 15,
            cooldown: 2,
            current_cooldown: 0,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::RangedAttack,
        },
        BossAbility {
            name: "Mind Control".to_string(),
            name_zh: "精神控制".to_string(),
            damage: 0,
            cooldown: 5,
            current_cooldown: 2,
            effect: Some(StatusEffectKind::Stunned),
            effect_duration: 2,
            ability_type: BossAbilityType::Debuff,
        },
    ];

    if phase >= 2 {
        abilities.push(BossAbility {
            name: "Reality Warp".to_string(),
            name_zh: "现实扭曲".to_string(),
            damage: 0,
            cooldown: 8,
            current_cooldown: 4,
            effect: Some(StatusEffectKind::Weakened),
            effect_duration: 4,
            ability_type: BossAbilityType::Debuff,
        });
        abilities.push(BossAbility {
            name: "Summon Horror".to_string(),
            name_zh: "召唤恐怖".to_string(),
            damage: 0,
            cooldown: 7,
            current_cooldown: 3,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::Summon,
        });
    }

    if phase >= 3 {
        abilities.push(BossAbility {
            name: "Void Devourer".to_string(),
            name_zh: "虚空吞噬".to_string(),
            damage: 120,
            cooldown: 15,
            current_cooldown: 10,
            effect: None,
            effect_duration: 0,
            ability_type: BossAbilityType::AreaAttack,
        });
    }

    abilities
}
