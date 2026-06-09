use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    CharacterSelect,
    SeedInput,
    GeneratingMap,
    Playing,
    PlayerTurn,
    EnemyTurn,
    GameOver,
    Victory,
    Paused,
    Inventory,
    Achievements,
    Settings,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CombatState {
    #[default]
    Idle,
    PlayerInput,
    PlayerAction,
    EnemyAction,
    TurnEnd,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RenderMode {
    #[default]
    Ascii,
    Tile,
}
