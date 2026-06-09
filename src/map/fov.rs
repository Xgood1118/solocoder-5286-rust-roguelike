use bevy::prelude::*;
use crate::components::*;
use super::map::GameMap;

const MULT: [[i32; 8]; 8] = [
    [1, 0, 0, -1, -1, 0, 0, 1],
    [0, 1, -1, 0, 0, -1, 1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [1, 0, 0, 1, -1, 0, 0, -1],
    [0, 0, 1, 1, 0, 0, -1, -1],
    [0, 0, 1, -1, 0, 0, -1, 1],
    [1, 1, 0, 0, -1, -1, 0, 0],
    [1, -1, 0, 0, -1, 1, 0, 0],
];

pub fn update_fov(
    mut map: ResMut<GameMap>,
    mut query: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
) {
    map.clear_visibility();

    for (pos, mut fov, is_player) in query.iter_mut() {
        if !fov.dirty && is_player.is_none() {
            continue;
        }

        fov.visible.clear();
        fov.visible.push(*pos);

        for octant in 0..8 {
            cast_light(
                &mut map,
                pos.x,
                pos.y,
                fov.range,
                1,
                1.0,
                0.0,
                octant,
                &mut fov.visible,
            );
        }

        fov.dirty = false;
    }
}

fn cast_light(
    map: &mut GameMap,
    cx: i32,
    cy: i32,
    radius: i32,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    octant: usize,
    visible: &mut Vec<Position>,
) {
    if start_slope < end_slope {
        return;
    }

    let mut next_start_slope = start_slope;

    for y in row..=radius {
        let mut blocked = false;

        for dx in -y..=0 {
            let dy = -dx as f32 / y as f32;

            if dy > start_slope {
                continue;
            }
            if dy < end_slope {
                break;
            }

            let map_x = cx + dx * MULT[octant][0] + y * MULT[octant][1];
            let map_y = cy + dx * MULT[octant][2] + y * MULT[octant][3];

            if !map.in_bounds(map_x, map_y) {
                continue;
            }

            let distance = (dx * dx + y * y) as f32;
            if distance <= (radius * radius) as f32 {
                let pos = Position::new(map_x, map_y);
                if !visible.contains(&pos) {
                    visible.push(pos);
                }
                map.set_visible(map_x, map_y);
            }

            let tile = map.get_tile(map_x, map_y).unwrap();
            if tile.blocks_vision {
                if !blocked && y < radius {
                    blocked = true;
                    let new_end = (dx as f32 - 0.5) / (y as f32 + 0.5);
                    if new_end < end_slope {
                        break;
                    }
                    cast_light(map, cx, cy, radius, y + 1, next_start_slope, new_end, octant, visible);
                }
                next_start_slope = (dx as f32 + 0.5) / (y as f32 - 0.5);
            } else if blocked {
                blocked = false;
                next_start_slope = (dx as f32 - 0.5) / (y as f32 - 0.5);
            }
        }

        if blocked {
            break;
        }
    }
}

pub fn field_of_view(map: &GameMap, x: i32, y: i32, radius: i32) -> Vec<Position> {
    let mut visible = Vec::new();
    let mut map_clone = map.clone();

    visible.push(Position::new(x, y));
    map_clone.set_visible(x, y);

    for octant in 0..8 {
        cast_light(&mut map_clone, x, y, radius, 1, 1.0, 0.0, octant, &mut visible);
    }

    visible
}
