use crate::components::Position;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
    Door,
    Trap,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub explored: bool,
    pub visible: bool,
    pub blocks_movement: bool,
    pub blocks_vision: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let (blocks_movement, blocks_vision) = match tile_type {
            TileType::Wall => (true, true),
            TileType::Floor => (false, false),
            TileType::DownStairs => (false, false),
            TileType::UpStairs => (false, false),
            TileType::Door => (false, false),
            TileType::Trap => (false, false),
        };

        Self {
            tile_type,
            explored: false,
            visible: false,
            blocks_movement,
            blocks_vision,
        }
    }

    pub fn wall() -> Self {
        Self::new(TileType::Wall)
    }

    pub fn floor() -> Self {
        Self::new(TileType::Floor)
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
}

impl GameMap {
    pub fn from_tiles(tiles: Vec<Tile>, width: i32, height: i32) -> Self {
        Self { tiles, width, height }
    }

    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![Tile::wall(); (width * height) as usize];
        Self { tiles, width, height }
    }

    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if self.in_bounds(x, y) {
            Some(&self.tiles[self.idx(x, y)])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if self.in_bounds(x, y) {
            let idx = self.idx(x, y);
            Some(&mut self.tiles[idx])
        } else {
            None
        }
    }

    pub fn is_occupied(&self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.blocks_movement
        } else {
            true
        }
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.visible
        } else {
            false
        }
    }

    pub fn is_explored(&self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.explored
        } else {
            false
        }
    }

    pub fn clear_visibility(&mut self) {
        for tile in &mut self.tiles {
            tile.visible = false;
        }
    }

    pub fn reveal_all(&mut self) {
        for tile in &mut self.tiles {
            tile.visible = true;
            tile.explored = true;
        }
    }

    pub fn set_visible(&mut self, x: i32, y: i32) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            tile.visible = true;
            tile.explored = true;
        }
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.tile_type == TileType::Wall
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, width: w, height: h }
    }

    pub fn center(&self) -> Position {
        Position::new(
            self.x + self.width / 2,
            self.y + self.height / 2,
        )
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x <= other.x + other.width
            && self.x + self.width >= other.x
            && self.y <= other.y + other.height
            && self.y + self.height >= other.y
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct Rooms {
    pub rooms: Vec<Rect>,
}

impl Rooms {
    pub fn new(rooms: Vec<Rect>) -> Self {
        Self { rooms }
    }

    pub fn len(&self) -> usize {
        self.rooms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Rect> {
        self.rooms.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Rect> {
        self.rooms.get(index)
    }

    pub fn first(&self) -> Option<&Rect> {
        self.rooms.first()
    }

    pub fn last(&self) -> Option<&Rect> {
        self.rooms.last()
    }
}
