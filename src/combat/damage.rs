use crate::components::Stats;
use rand::Rng;

pub struct DamageResult {
    pub total: i32,
    pub base: i32,
    pub is_crit: bool,
    pub crit_multiplier: f32,
}

pub fn calculate_damage<R: Rng>(
    attacker_stats: &Stats,
    defender_stats: &Stats,
    rng: &mut R,
) -> DamageResult {
    let base_damage = (attacker_stats.attack - defender_stats.defense / 2).max(1);

    let is_crit = rng.gen::<f32>() < attacker_stats.crit_rate;
    let crit_multiplier = if is_crit {
        attacker_stats.crit_damage
    } else {
        1.0
    };

    let variance = rng.gen_range(-0.1..=0.1);
    let total = (base_damage as f32 * crit_multiplier * (1.0 + variance)) as i32;

    DamageResult {
        total: total.max(1),
        base: base_damage,
        is_crit,
        crit_multiplier,
    }
}

pub fn calculate_spell_damage<R: Rng>(
    spell_power: i32,
    defender_resist: i32,
    crit_rate: f32,
    crit_damage: f32,
    rng: &mut R,
) -> DamageResult {
    let base_damage = (spell_power - defender_resist / 3).max(1);

    let is_crit = rng.gen::<f32>() < crit_rate;
    let multiplier = if is_crit { crit_damage } else { 1.0 };

    let variance = rng.gen_range(-0.15..=0.15);
    let total = (base_damage as f32 * multiplier * (1.0 + variance)) as i32;

    DamageResult {
        total: total.max(1),
        base: base_damage,
        is_crit,
        crit_multiplier: multiplier,
    }
}

pub fn calculate_heal_amount<R: Rng>(
    base_heal: i32,
    spell_power: i32,
    rng: &mut R,
) -> i32 {
    let heal = base_heal + spell_power / 2;
    let variance = rng.gen_range(-0.1..=0.1);
    (heal as f32 * (1.0 + variance)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand::SeedableRng;

    #[test]
    fn test_damage_basic() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Stats {
            attack: 20,
            defense: 0,
            speed: 10,
            crit_rate: 0.0,
            crit_damage: 2.0,
        };
        let defender = Stats {
            attack: 0,
            defense: 10,
            speed: 10,
            crit_rate: 0.0,
            crit_damage: 2.0,
        };

        let result = calculate_damage(&attacker, &defender, &mut rng);
        assert!(result.total >= 1);
        assert!(!result.is_crit);
    }

    #[test]
    fn test_crit_damage() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Stats {
            attack: 100,
            defense: 0,
            speed: 10,
            crit_rate: 1.0,
            crit_damage: 2.0,
        };
        let defender = Stats {
            attack: 0,
            defense: 0,
            speed: 10,
            crit_rate: 0.0,
            crit_damage: 2.0,
        };

        let result = calculate_damage(&attacker, &defender, &mut rng);
        assert!(result.is_crit);
        assert_eq!(result.crit_multiplier, 2.0);
    }

    #[test]
    fn test_minimum_damage() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let attacker = Stats {
            attack: 1,
            defense: 0,
            speed: 10,
            crit_rate: 0.0,
            crit_damage: 2.0,
        };
        let defender = Stats {
            attack: 0,
            defense: 100,
            speed: 10,
            crit_rate: 0.0,
            crit_damage: 2.0,
        };

        let result = calculate_damage(&attacker, &defender, &mut rng);
        assert_eq!(result.total, 1);
    }
}
