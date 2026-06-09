use bevy::prelude::*;
use crate::components::*;
use crate::bundles::*;
use crate::map::map::Rooms;
use rand::Rng;

pub fn spawn_player(
    commands: &mut Commands,
    rooms: &Rooms,
    class: CharacterClass,
) -> Entity {
    let start_pos = rooms.first().unwrap().center();

    let base_stats = class.base_stats();
    let base_health = class.base_health();

    commands.spawn(PlayerBundle {
        player: Player,
        name: EntityName(class.name().to_string()),
        position: start_pos,
        health: Health::new(base_health),
        energy: Energy::new(100),
        stats: base_stats,
        fov: FieldOfView::new(8),
        glyph: Glyph {
            symbol: class.glyph(),
            color: Color::YELLOW,
        },
        class: PlayerClass {
            class,
            level: 1,
            experience: 0,
            experience_to_next: 100,
        },
        blocks_movement: BlocksMovement,
        combatant: Combatant,
    }).id()
}

pub fn level_up(player: &mut PlayerClass, stats: &mut Stats, health: &mut Health) {
    player.level += 1;
    player.experience -= player.experience_to_next;
    player.experience_to_next = (player.experience_to_next as f32 * 1.5) as i32;

    stats.attack += 2;
    stats.defense += 1;
    stats.speed += 1;

    health.max += 10;
    health.current = health.max;
}

pub fn gain_experience(
    player: &mut PlayerClass,
    stats: &mut Stats,
    health: &mut Health,
    exp: i32,
) -> i32 {
    let mut levels_gained = 0;
    player.experience += exp;

    while player.experience >= player.experience_to_next {
        level_up(player, stats, health);
        levels_gained += 1;
    }

    levels_gained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_experience() {
        let mut player_class = PlayerClass {
            class: CharacterClass::Warrior,
            level: 1,
            experience: 0,
            experience_to_next: 100,
        };
        let mut stats = Stats::default();
        let mut health = Health::new(100);

        let levels = gain_experience(&mut player_class, &mut stats, &mut health, 250);
        assert_eq!(levels, 2);
        assert_eq!(player_class.level, 3);
        assert!(player_class.experience < player_class.experience_to_next);
    }
}
