mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub(crate) struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Inventory>()
            .add_startup_system(ui::setup_ui);
    }
}

#[derive(Component)]
pub struct Pickupable;

#[derive(Component, Inspectable)]
pub enum ItemType {
    None,
    Chest,
    Leg,
    Bone,
    Arm,
}

impl Default for ItemType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Inspectable, Default)]
struct InventoryEntry {
    item: ItemType,
    count: usize,
}

#[derive(Component, Inspectable)]
pub struct Inventory {
    items: [InventoryEntry; 4],
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: [
                InventoryEntry {
                    item: ItemType::Bone,
                    count: 0,
                },
                InventoryEntry {
                    item: ItemType::Arm,
                    count: 0,
                },
                InventoryEntry {
                    item: ItemType::Leg,
                    count: 0,
                },
                InventoryEntry {
                    item: ItemType::Chest,
                    count: 0,
                },
            ],
        }
    }
}
