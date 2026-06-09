use crate::components::*;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixTemplate {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    pub slot_type: AffixSlotType,
    pub tier: i32,
    pub rarity: Rarity,
    pub stats_bonus: Stats,
    pub effect: Option<StatusEffectKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AffixSlotType {
    Prefix,
    Suffix,
    Any,
}

pub fn get_affix_templates() -> Vec<AffixTemplate> {
    vec![
        AffixTemplate {
            id: "fiery".to_string(),
            name: "Fiery".to_string(),
            name_zh: "灼热".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 3,
                defense: 0,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: Some(StatusEffectKind::Burning),
        },
        AffixTemplate {
            id: "frost".to_string(),
            name: "Frost".to_string(),
            name_zh: "冰霜".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 2,
                defense: 2,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: Some(StatusEffectKind::Frozen),
        },
        AffixTemplate {
            id: "poisoned".to_string(),
            name: "Poisoned".to_string(),
            name_zh: "剧毒".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 2,
                defense: 0,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: Some(StatusEffectKind::Poison),
        },
        AffixTemplate {
            id: "sharp".to_string(),
            name: "Sharp".to_string(),
            name_zh: "锋利".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 5,
                defense: 0,
                speed: 0,
                crit_rate: 0.03,
                crit_damage: 0.0,
            },
            effect: None,
        },
        AffixTemplate {
            id: "swift".to_string(),
            name: "Swift".to_string(),
            name_zh: "迅捷".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 0,
                defense: 0,
                speed: 3,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: None,
        },
        AffixTemplate {
            id: "mighty".to_string(),
            name: "Mighty".to_string(),
            name_zh: "强力".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 2,
            rarity: Rarity::Uncommon,
            stats_bonus: Stats {
                attack: 8,
                defense: 2,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: None,
        },
        AffixTemplate {
            id: "deadly".to_string(),
            name: "Deadly".to_string(),
            name_zh: "致命".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 3,
            rarity: Rarity::Rare,
            stats_bonus: Stats {
                attack: 10,
                defense: 0,
                speed: 0,
                crit_rate: 0.10,
                crit_damage: 0.2,
            },
            effect: None,
        },
        AffixTemplate {
            id: "of_the_tiger".to_string(),
            name: "of the Tiger".to_string(),
            name_zh: "之虎".to_string(),
            slot_type: AffixSlotType::Suffix,
            tier: 2,
            rarity: Rarity::Uncommon,
            stats_bonus: Stats {
                attack: 6,
                defense: 0,
                speed: 2,
                crit_rate: 0.05,
                crit_damage: 0.0,
            },
            effect: None,
        },
        AffixTemplate {
            id: "of_protection".to_string(),
            name: "of Protection".to_string(),
            name_zh: "防护".to_string(),
            slot_type: AffixSlotType::Suffix,
            tier: 1,
            rarity: Rarity::Common,
            stats_bonus: Stats {
                attack: 0,
                defense: 5,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: Some(StatusEffectKind::Shielded),
        },
        AffixTemplate {
            id: "of_vampirism".to_string(),
            name: "of Vampirism".to_string(),
            name_zh: "吸血".to_string(),
            slot_type: AffixSlotType::Suffix,
            tier: 3,
            rarity: Rarity::Rare,
            stats_bonus: Stats {
                attack: 5,
                defense: 0,
                speed: 0,
                crit_rate: 0.0,
                crit_damage: 0.0,
            },
            effect: None,
        },
        AffixTemplate {
            id: "godlike".to_string(),
            name: "Godlike".to_string(),
            name_zh: "神级".to_string(),
            slot_type: AffixSlotType::Prefix,
            tier: 5,
            rarity: Rarity::Legendary,
            stats_bonus: Stats {
                attack: 20,
                defense: 10,
                speed: 5,
                crit_rate: 0.10,
                crit_damage: 0.5,
            },
            effect: None,
        },
        AffixTemplate {
            id: "of_doom".to_string(),
            name: "of Doom".to_string(),
            name_zh: "末日".to_string(),
            slot_type: AffixSlotType::Suffix,
            tier: 5,
            rarity: Rarity::Legendary,
            stats_bonus: Stats {
                attack: 15,
                defense: 0,
                speed: 0,
                crit_rate: 0.15,
                crit_damage: 0.8,
            },
            effect: None,
        },
    ]
}

pub struct GeneratedItem {
    pub name: String,
    pub name_zh: String,
    pub stats: Stats,
    pub rarity: Rarity,
    pub affixes: Vec<Affix>,
}

pub fn generate_random_item<R: Rng>(
    base_name: &str,
    base_name_zh: &str,
    base_stats: &Stats,
    base_rarity: Rarity,
    rng: &mut R,
) -> GeneratedItem {
    let affix_templates = get_affix_templates();
    let mut affixes: Vec<Affix> = Vec::new();
    let mut total_stats = base_stats.clone();
    let mut final_rarity = base_rarity;

    let num_affixes = match base_rarity {
        Rarity::Common => 0,
        Rarity::Uncommon => rng.gen_range(0..=1),
        Rarity::Rare => rng.gen_range(1..=2),
        Rarity::Epic => rng.gen_range(2..=3),
        Rarity::Legendary => rng.gen_range(3..=4),
    };

    let prefixes: Vec<&AffixTemplate> = affix_templates
        .iter()
        .filter(|a| a.slot_type == AffixSlotType::Prefix || a.slot_type == AffixSlotType::Any)
        .filter(|a| a.rarity <= final_rarity)
        .collect();

    let suffixes: Vec<&AffixTemplate> = affix_templates
        .iter()
        .filter(|a| a.slot_type == AffixSlotType::Suffix || a.slot_type == AffixSlotType::Any)
        .filter(|a| a.rarity <= final_rarity)
        .collect();

    let mut has_prefix = false;
    let mut has_suffix = false;

    for _ in 0..num_affixes {
        let use_prefix = if !has_prefix && !has_suffix {
            rng.gen_bool(0.5)
        } else if !has_prefix {
            true
        } else if !has_suffix {
            false
        } else {
            rng.gen_bool(0.5)
        };

        let template = if use_prefix && !prefixes.is_empty() {
            has_prefix = true;
            prefixes.choose(rng)
        } else if !suffixes.is_empty() {
            has_suffix = true;
            suffixes.choose(rng)
        } else {
            None
        };

        if let Some(affix_template) = template {
            let affix = Affix {
                name: affix_template.name.clone(),
                stats_bonus: affix_template.stats_bonus.clone(),
                tier: affix_template.tier,
            };

            total_stats.attack += affix.stats_bonus.attack;
            total_stats.defense += affix.stats_bonus.defense;
            total_stats.speed += affix.stats_bonus.speed;
            total_stats.crit_rate += affix.stats_bonus.crit_rate;
            total_stats.crit_damage += affix.stats_bonus.crit_damage;

            if affix_template.rarity > final_rarity {
                final_rarity = affix_template.rarity;
            }

            affixes.push(affix);
        }
    }

    let (full_name, full_name_zh) = build_item_name(base_name, base_name_zh, &affixes);

    GeneratedItem {
        name: full_name,
        name_zh: full_name_zh,
        stats: total_stats,
        rarity: final_rarity,
        affixes,
    }
}

fn build_item_name(base_name: &str, base_name_zh: &str, affixes: &[Affix]) -> (String, String) {
    let mut name_en = String::new();
    let mut name_zh = String::new();

    for affix in affixes {
        if name_en.is_empty() {
            name_en.push_str(&affix.name);
            name_zh.push_str(&affix.name);
        } else {
            name_en.push(' ');
            name_en.push_str(&affix.name);
        }
    }

    if !name_en.is_empty() {
        name_en.push(' ');
    }
    name_en.push_str(base_name);

    if !name_zh.is_empty() {
        name_zh.push_str(base_name_zh);
    } else {
        name_zh = base_name_zh.to_string();
    }

    (name_en, name_zh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand::SeedableRng;

    #[test]
    fn test_generate_common_item() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let base_stats = Stats::default();

        let result = generate_random_item(
            "Sword",
            "剑",
            &base_stats,
            Rarity::Common,
            &mut rng,
        );

        assert_eq!(result.rarity, Rarity::Common);
        assert_eq!(result.name, "Sword");
    }

    #[test]
    fn test_generate_rare_item() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let base_stats = Stats::default();

        let result = generate_random_item(
            "Sword",
            "剑",
            &base_stats,
            Rarity::Rare,
            &mut rng,
        );

        assert!(result.affixes.len() >= 1);
        assert!(result.affixes.len() <= 2);
        assert!(!result.name.is_empty());
    }
}
