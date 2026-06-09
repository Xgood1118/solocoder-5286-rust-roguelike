pub mod player;
pub mod monsters;
pub mod items;
pub mod affixes;

use bevy::prelude::*;
use crate::states::*;
use crate::components::*;
use crate::map::map::{Rooms, GameMap};
use crate::resources::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_initial_entities);
    }
}

fn spawn_initial_entities(
    mut commands: Commands,
    rooms: Res<Rooms>,
    game_seed: Res<GameSeed>,
    mut rng: ResMut<GameRng>,
    mut map: ResMut<GameMap>,
) {
    map.reveal_all();

    player::spawn_player(
        &mut commands,
        &rooms,
        CharacterClass::Warrior,
    );

    monsters::spawn_monsters_for_floor(
        &mut commands,
        &rooms,
        game_seed.level,
        &mut rng.rng,
    );
}
