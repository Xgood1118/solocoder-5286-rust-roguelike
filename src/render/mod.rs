use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::map::map::GameMap;
use crate::resources::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileConfig>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (
                render_ascii_map,
                render_entities,
                render_ui,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct TileConfig {
    pub tile_size: f32,
    pub font_size: f32,
    pub camera_zoom: f32,
}

impl Default for TileConfig {
    fn default() -> Self {
        Self {
            tile_size: 16.0,
            font_size: 16.0,
            camera_zoom: 1.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn render_ascii_map(
    mut commands: Commands,
    map: Res<GameMap>,
    tile_config: Res<TileConfig>,
    tile_query: Query<Entity, With<MapTileMarker>>,
    player_query: Query<&Position, With<Player>>,
) {
    for entity in tile_query.iter() {
        commands.entity(entity).despawn();
    }

    let _camera_offset = if let Ok(player_pos) = player_query.get_single() {
        Vec2::new(
            player_pos.x as f32 * tile_config.tile_size,
            player_pos.y as f32 * tile_config.tile_size,
        )
    } else {
        Vec2::ZERO
    };

    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(tile) = map.get_tile(x, y) {
                if !tile.explored {
                    continue;
                }

                let (symbol, color) = get_tile_glyph(tile);
                let visibility = if tile.visible { 1.0 } else { 0.5 };

                let screen_x = (x as f32 - map.width as f32 / 2.0) * tile_config.tile_size;
                let screen_y = (-(y as f32) + map.height as f32 / 2.0) * tile_config.tile_size;

                commands
                    .spawn((
                        MapTileMarker,
                        Text2dBundle {
                            text: Text::from_section(
                                symbol.to_string(),
                                TextStyle {
                                    font_size: tile_config.font_size,
                                    color: color.with_a(visibility),
                                    ..default()
                                },
                            ),
                            transform: Transform::from_xyz(screen_x, screen_y, 0.0),
                            ..default()
                        },
                    ));
            }
        }
    }
}

fn get_tile_glyph(tile: &crate::map::map::Tile) -> (char, Color) {
    match tile.tile_type {
        crate::map::map::TileType::Wall => ('#', Color::DARK_GRAY),
        crate::map::map::TileType::Floor => ('.', Color::GRAY),
        crate::map::map::TileType::DownStairs => ('>', Color::YELLOW),
        crate::map::map::TileType::UpStairs => ('<', Color::YELLOW),
        crate::map::map::TileType::Door => ('+', Color::rgb(0.6, 0.4, 0.2)),
        crate::map::map::TileType::Trap => ('^', Color::RED),
    }
}

#[derive(Component, Debug, Clone)]
pub struct MapTileMarker;

#[derive(Component, Debug, Clone)]
pub struct EntitySpriteMarker;

fn render_entities(
    mut commands: Commands,
    entity_query: Query<(&Position, &Glyph), With<Combatant>>,
    item_query: Query<(&Position, &Glyph), (With<Item>, With<OnGround>)>,
    map: Res<GameMap>,
    tile_config: Res<TileConfig>,
    sprite_query: Query<Entity, With<EntitySpriteMarker>>,
) {
    for entity in sprite_query.iter() {
        commands.entity(entity).despawn();
    }

    let mut renderables: Vec<(Position, Glyph)> = Vec::new();

    for (pos, glyph) in item_query.iter() {
        if map.is_visible(pos.x, pos.y) {
            renderables.push((*pos, glyph.clone()));
        }
    }

    for (pos, glyph) in entity_query.iter() {
        if map.is_visible(pos.x, pos.y) {
            renderables.push((*pos, glyph.clone()));
        }
    }

    for (pos, glyph) in renderables {
        let screen_x = (pos.x as f32 - map.width as f32 / 2.0) * tile_config.tile_size;
        let screen_y = (-(pos.y as f32) + map.height as f32 / 2.0) * tile_config.tile_size;

        commands.spawn((
            EntitySpriteMarker,
            Text2dBundle {
                text: Text::from_section(
                    glyph.symbol.to_string(),
                    TextStyle {
                        font_size: tile_config.font_size,
                        color: glyph.color,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(screen_x, screen_y, 10.0),
                ..default()
            },
        ));
    }
}

fn render_ui(
    _commands: Commands,
    player_query: Query<(&Health, &Energy, &Stats, &PlayerClass), With<Player>>,
    _turn_counter: Res<TurnCounter>,
    _game_seed: Res<GameSeed>,
) {
    if let Ok((_health, _energy, _stats, _class)) = player_query.get_single() {
    }
}
