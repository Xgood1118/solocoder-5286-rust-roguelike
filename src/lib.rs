pub mod components;
pub mod bundles;
pub mod resources;
pub mod states;
pub mod map;
pub mod combat;
pub mod entities;
pub mod ai;
pub mod inventory;
pub mod persistence;
pub mod render;
pub mod achievements;
pub mod i18n;

use bevy::prelude::*;

pub struct RoguelikePlugin;

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            map::MapPlugin,
            combat::CombatPlugin,
            entities::EntitiesPlugin,
            ai::AiPlugin,
            inventory::InventoryPlugin,
            persistence::PersistencePlugin,
            render::RenderPlugin,
            achievements::AchievementPlugin,
            i18n::I18nPlugin,
        ));
    }
}
