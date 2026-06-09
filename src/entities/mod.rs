pub mod player;
pub mod monsters;
pub mod items;
pub mod affixes;

use bevy::prelude::*;
use crate::states::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_initial_entities);
    }
}

fn spawn_initial_entities() {
}
