use bevy::prelude::*;
use crate::components::*;
use crate::states::*;
use crate::resources::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            pickup_items,
            drop_item,
            equip_item,
            unequip_item,
            update_equipped_stats,
        ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Inventory {
    pub width: i32,
    pub height: i32,
    pub items: Vec<Option<Entity>>,
}

impl Inventory {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            items: vec![None; size],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.items[idx]
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, item: Option<Entity>) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.items[idx] = item;
        }
    }

    pub fn find_empty_slot(&self) -> Option<(i32, i32)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y).is_none() {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn add_item(&mut self, item_entity: Entity) -> bool {
        if let Some((x, y)) = self.find_empty_slot() {
            self.set(x, y, Some(item_entity));
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_entity: Entity) -> bool {
        for i in 0..self.items.len() {
            if self.items[i] == Some(item_entity) {
                self.items[i] = None;
                return true;
            }
        }
        false
    }

    pub fn count_items(&self) -> usize {
        self.items.iter().filter(|i| i.is_some()).count()
    }

    pub fn is_full(&self) -> bool {
        self.count_items() >= self.items.len()
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct Equipment {
    pub weapon: Option<Entity>,
    pub offhand: Option<Entity>,
    pub head: Option<Entity>,
    pub chest: Option<Entity>,
    pub hands: Option<Entity>,
    pub feet: Option<Entity>,
    pub accessory1: Option<Entity>,
    pub accessory2: Option<Entity>,
}

impl Equipment {
    pub fn get_slot(&self, slot: EquipmentSlotKind) -> Option<Entity> {
        match slot {
            EquipmentSlotKind::Weapon => self.weapon,
            EquipmentSlotKind::Offhand => self.offhand,
            EquipmentSlotKind::Head => self.head,
            EquipmentSlotKind::Chest => self.chest,
            EquipmentSlotKind::Hands => self.hands,
            EquipmentSlotKind::Feet => self.feet,
            EquipmentSlotKind::Accessory1 => self.accessory1,
            EquipmentSlotKind::Accessory2 => self.accessory2,
        }
    }

    pub fn set_slot(&mut self, slot: EquipmentSlotKind, item: Option<Entity>) {
        match slot {
            EquipmentSlotKind::Weapon => self.weapon = item,
            EquipmentSlotKind::Offhand => self.offhand = item,
            EquipmentSlotKind::Head => self.head = item,
            EquipmentSlotKind::Chest => self.chest = item,
            EquipmentSlotKind::Hands => self.hands = item,
            EquipmentSlotKind::Feet => self.feet = item,
            EquipmentSlotKind::Accessory1 => self.accessory1 = item,
            EquipmentSlotKind::Accessory2 => self.accessory2 = item,
        }
    }

    pub fn iter(&self) -> EquipmentIter {
        EquipmentIter {
            slots: vec![
                (EquipmentSlotKind::Weapon, self.weapon),
                (EquipmentSlotKind::Offhand, self.offhand),
                (EquipmentSlotKind::Head, self.head),
                (EquipmentSlotKind::Chest, self.chest),
                (EquipmentSlotKind::Hands, self.hands),
                (EquipmentSlotKind::Feet, self.feet),
                (EquipmentSlotKind::Accessory1, self.accessory1),
                (EquipmentSlotKind::Accessory2, self.accessory2),
            ],
            index: 0,
        }
    }
}

pub struct EquipmentIter {
    slots: Vec<(EquipmentSlotKind, Option<Entity>)>,
    index: usize,
}

impl Iterator for EquipmentIter {
    type Item = (EquipmentSlotKind, Option<Entity>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slots.len() {
            let item = self.slots[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn pickup_items(
    mut commands: Commands,
    player_query: Query<(Entity, &Position), With<Player>>,
    item_query: Query<(Entity, &Position), (With<Item>, With<OnGround>)>,
    mut inventory: ResMut<Inventory>,
    mut player_stats: ResMut<PlayerStats>,
) {
    if let Ok((player_entity, player_pos)) = player_query.get_single() {
        let mut to_pickup = Vec::new();

        for (item_entity, item_pos) in item_query.iter() {
            if player_pos == item_pos {
                to_pickup.push(item_entity);
            }
        }

        for item_entity in to_pickup {
            if inventory.add_item(item_entity) {
                commands.entity(item_entity).remove::<OnGround>();
                commands.entity(item_entity).remove::<Position>();
                commands.entity(item_entity).insert(CarriedBy(player_entity));
                player_stats.items_picked_up += 1;
            }
        }
    }
}

fn drop_item(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    item_query: Query<Entity, With<Item>>,
    player_query: Query<&Position, With<Player>>,
    mut drop_events: EventReader<DropItemEvent>,
) {
    for event in drop_events.read() {
        if item_query.get(event.item).is_ok() {
            if inventory.remove_item(event.item) {
                if let Ok(player_pos) = player_query.get_single() {
                    commands.entity(event.item).remove::<CarriedBy>();
                    commands.entity(event.item).remove::<Equipped>();
                    commands.entity(event.item).insert(OnGround);
                    commands.entity(event.item).insert(*player_pos);
                }
            }
        }
    }
}

fn equip_item(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    mut equipment: ResMut<Equipment>,
    item_query: Query<&ItemBase, With<Item>>,
    mut equip_events: EventReader<EquipItemEvent>,
) {
    for event in equip_events.read() {
        if let Ok(item_base) = item_query.get(event.item) {
            if inventory.remove_item(event.item) {
                let slot = get_slot_for_item(item_base);
                if let Some(current_item) = equipment.get_slot(slot) {
                    inventory.add_item(current_item);
                    commands.entity(current_item).remove::<Equipped>();
                }
                equipment.set_slot(slot, Some(event.item));
                commands.entity(event.item).insert(Equipped);
            }
        }
    }
}

fn unequip_item(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    mut equipment: ResMut<Equipment>,
    mut unequip_events: EventReader<UnequipItemEvent>,
) {
    for event in unequip_events.read() {
        if let Some(item_entity) = equipment.get_slot(event.slot) {
            if !inventory.is_full() {
                inventory.add_item(item_entity);
                commands.entity(item_entity).remove::<Equipped>();
                equipment.set_slot(event.slot, None);
            }
        }
    }
}

fn get_slot_for_item(item_base: &ItemBase) -> EquipmentSlotKind {
    match item_base.item_type {
        ItemType::Weapon => EquipmentSlotKind::Weapon,
        ItemType::Armor => EquipmentSlotKind::Chest,
        _ => EquipmentSlotKind::Accessory1,
    }
}

fn update_equipped_stats() {
}

#[derive(Event, Debug, Clone, Copy)]
pub struct DropItemEvent {
    pub item: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct EquipItemEvent {
    pub item: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct UnequipItemEvent {
    pub slot: EquipmentSlotKind,
}
