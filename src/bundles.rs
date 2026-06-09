use bevy::prelude::*;
use crate::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: EntityName,
    pub position: Position,
    pub health: Health,
    pub energy: Energy,
    pub stats: Stats,
    pub fov: FieldOfView,
    pub glyph: Glyph,
    pub class: PlayerClass,
    pub blocks_movement: BlocksMovement,
    pub combatant: Combatant,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub name: EntityName,
    pub position: Position,
    pub health: Health,
    pub stats: Stats,
    pub fov: FieldOfView,
    pub glyph: Glyph,
    pub kind: EnemyKind,
    pub ai: AIControlled,
    pub blocks_movement: BlocksMovement,
    pub blocks_vision: BlocksVision,
    pub combatant: Combatant,
    pub turn_order: TurnOrder,
}

#[derive(Bundle)]
pub struct BossBundle {
    pub enemy: Enemy,
    pub boss: Boss,
    pub name: EntityName,
    pub position: Position,
    pub health: Health,
    pub stats: Stats,
    pub fov: FieldOfView,
    pub glyph: Glyph,
    pub kind: EnemyKind,
    pub ai: AIControlled,
    pub blocks_movement: BlocksMovement,
    pub blocks_vision: BlocksVision,
    pub combatant: Combatant,
    pub turn_order: TurnOrder,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub name: EntityName,
    pub position: Position,
    pub glyph: Glyph,
    pub item_base: ItemBase,
    pub on_ground: OnGround,
}

#[derive(Bundle)]
pub struct TileBundle {
    pub position: Position,
    pub glyph: Glyph,
    pub explored: Explored,
}

#[derive(Bundle)]
pub struct StairsBundle {
    pub position: Position,
    pub glyph: Glyph,
    pub down_stairs: DownStairs,
}
