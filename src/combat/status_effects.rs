use crate::components::*;
use bevy::prelude::*;

pub fn add_status_effect(
    commands: &mut Commands,
    target: Entity,
    effect: StatusEffectKind,
    duration: i32,
    magnitude: i32,
) {
    commands.entity(target).insert(StatusEffect {
        effect,
        duration,
        magnitude,
    });
}

pub fn get_stat_modifiers(effects: &[StatusEffect]) -> Stats {
    let mut modifiers = Stats::default();

    for effect in effects {
        match effect.effect {
            StatusEffectKind::Weakened => {
                modifiers.attack -= effect.magnitude;
            }
            StatusEffectKind::Enraged => {
                modifiers.attack += effect.magnitude;
            }
            StatusEffectKind::Shielded => {
                modifiers.defense += effect.magnitude;
            }
            StatusEffectKind::Slowed => {
                modifiers.speed -= effect.magnitude;
            }
            StatusEffectKind::Hasted => {
                modifiers.speed += effect.magnitude;
            }
            _ => {}
        }
    }

    modifiers
}

pub fn is_stunned(effects: &[StatusEffect]) -> bool {
    effects.iter().any(|e| e.effect == StatusEffectKind::Stunned)
}

pub fn is_invisible(effects: &[StatusEffect]) -> bool {
    effects.iter().any(|e| e.effect == StatusEffectKind::Invisible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_modifiers() {
        let effects = vec![
            StatusEffect {
                effect: StatusEffectKind::Enraged,
                duration: 3,
                magnitude: 5,
            },
            StatusEffect {
                effect: StatusEffectKind::Shielded,
                duration: 2,
                magnitude: 3,
            },
        ];

        let modifiers = get_stat_modifiers(&effects);
        assert_eq!(modifiers.attack, 5);
        assert_eq!(modifiers.defense, 3);
    }

    #[test]
    fn test_is_stunned() {
        let effects = vec![StatusEffect {
            effect: StatusEffectKind::Stunned,
            duration: 1,
            magnitude: 0,
        }];

        assert!(is_stunned(&effects));

        let empty: Vec<StatusEffect> = Vec::new();
        assert!(!is_stunned(&empty));
    }
}
