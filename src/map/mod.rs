pub mod bsp;
pub mod fov;
pub mod map;

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapConfig>()
            .add_systems(OnEnter(GameState::GeneratingMap), generate_map)
            .add_systems(Update, fov::update_fov.run_if(in_state(GameState::Playing)));
    }
}

fn generate_map(
    mut commands: Commands,
    config: Res<MapConfig>,
    mut game_seed: ResMut<GameSeed>,
    mut rng: ResMut<GameRng>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let map = bsp::generate_bsp_map(
        config.width,
        config.height,
        config.bsp_depth,
        config.min_room_size,
        &mut rng.rng,
    );

    commands.insert_resource(map::GameMap::from_tiles(map.tiles, config.width, config.height));
    commands.insert_resource(map::Rooms::new(map.rooms));

    next_state.set(GameState::Playing);
}
