mod ui;

use crate::creatures::{
    BoneTag, Creature, SceneModelState, ToDespawn, TypeCreature, ARMS_NEEDED_FULL_BODY,
    BONES_NEEDED_FULL_BODY, BONES_NEEDED_HALF_BODY, CHEST_NEEDED_FULL_BODY, CHEST_NEEDED_HALF_BODY,
    LEGS_NEEDED_FULL_BODY, LEGS_NEEDED_HALF_BODY,
};
use crate::inventory::ui::InventoryTextTag;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::CollisionEvent;

pub const STARTING_NB_BONES: usize = 9;
pub const STARTING_NB_CHEST: usize = 0;
pub const STARTING_NB_ARM: usize = 0;
pub const STARTING_NB_LEG: usize = 0;

pub static TEXT_INV_BONE: &str = "Bone:";
pub static TEXT_INV_CHEST: &str = "Chest:";
pub static TEXT_INV_ARM: &str = "Arm:";
pub static TEXT_INV_LEG: &str = "Leg:";

pub(crate) struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            //.register_inspectable::<Inventory>()
            .add_startup_system(ui::setup_ui)
            .add_system(update_inventory_text)
            .add_system_to_stage(CoreStage::PostUpdate, update_inventory_on_pickup)
            .add_system(update_game_state_on_inventory);
    }
}

#[derive(Component)]
pub struct Pickupable;

#[derive(Component, Eq, PartialEq, Inspectable)]
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

impl ItemType {
    fn get_text(&self) -> String {
        match self {
            ItemType::None => String::default(),
            ItemType::Bone => String::from(TEXT_INV_BONE),
            ItemType::Chest => String::from(TEXT_INV_CHEST),
            ItemType::Arm => String::from(TEXT_INV_ARM),
            ItemType::Leg => String::from(TEXT_INV_LEG),
        }
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

impl Inventory {
    pub(crate) fn add_bone(&mut self, count: usize) {
        self.items[0].count += count;
    }

    pub(crate) fn add_arms(&mut self, count: usize) {
        self.items[1].count += count;
    }

    pub(crate) fn add_legs(&mut self, count: usize) {
        self.items[2].count += count;
    }

    pub(crate) fn add_chest(&mut self, count: usize) {
        self.items[3].count += count;
    }

    fn number_of_bones(&self) -> usize {
        self.items[0].count
    }

    fn number_of_arms(&self) -> usize {
        self.items[1].count
    }

    fn number_of_legs(&self) -> usize {
        self.items[2].count
    }

    fn number_of_chest(&self) -> usize {
        self.items[3].count
    }

    fn has_enough_to_upgrade(&self, type_creature: TypeCreature) -> bool {
        if !(type_creature == TypeCreature::SkellyHalf
            || type_creature == TypeCreature::SkellyFullBody)
        {
            error!("Calling to upgrade with bad argument : {:?}", type_creature);
            return false;
        }

        let bones_count = self.number_of_bones();
        let chest_count = self.number_of_chest();
        let arms_count = self.number_of_arms();
        let legs_count = self.number_of_legs();

        if type_creature == TypeCreature::SkellyHalf {
            if bones_count >= BONES_NEEDED_HALF_BODY
                && chest_count >= CHEST_NEEDED_HALF_BODY
                && legs_count >= LEGS_NEEDED_HALF_BODY
            {
                return true;
            }
            return false;
        }

        if type_creature == TypeCreature::SkellyFullBody {
            if bones_count >= BONES_NEEDED_FULL_BODY
                && chest_count >= CHEST_NEEDED_FULL_BODY
                && arms_count >= ARMS_NEEDED_FULL_BODY
                && legs_count >= LEGS_NEEDED_FULL_BODY
            {
                return true;
            }
            return false;
        }

        false
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: [
                InventoryEntry {
                    item: ItemType::Bone,
                    count: STARTING_NB_BONES,
                },
                InventoryEntry {
                    item: ItemType::Arm,
                    count: STARTING_NB_ARM,
                },
                InventoryEntry {
                    item: ItemType::Leg,
                    count: STARTING_NB_LEG,
                },
                InventoryEntry {
                    item: ItemType::Chest,
                    count: STARTING_NB_CHEST,
                },
            ],
        }
    }
}

/// Update inventory text on update (+ or - items)
/// Could make Changed<T> occurs on InventoryEntry.. so I had to query the whole Inventory
fn update_inventory_text(
    mut query_text: Query<(&mut Text, &InventoryTextTag)>,
    query_item: Query<&Inventory, Changed<Inventory>>,
) {
    if let Ok(inventory) = query_item.get_single() {
        for item in &inventory.items {
            for (mut text_section, text_tag) in query_text.iter_mut() {
                if text_tag.0 == item.item {
                    text_section.sections[0].value =
                        format!("{} {}", item.item.get_text(), item.count);
                }
            }
        }
    }
}

fn update_inventory_on_pickup(
    parent_query: Query<&Parent>,
    query_bone: Query<&Creature, With<BoneTag>>,
    mut query_inventory: Query<&mut Inventory>,
    mut command: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(_skelly_child, entity_bone_child, _) = collision_event {
            // despawn bone (need parent because collider is inside child)

            for entity in [_skelly_child, entity_bone_child] {
                if let Ok(entity_bone) = parent_query.get(*entity) {
                    if let Ok(bone_creature) = query_bone.get(entity_bone.get()) {
                        command.entity(entity_bone.get()).insert(ToDespawn);

                        // Add bone to inventory count
                        if let Ok(mut inventory) = query_inventory.get_single_mut() {
                            match bone_creature.type_creature {
                                TypeCreature::Chest => inventory.add_chest(1),
                                TypeCreature::Leg => inventory.add_legs(1),
                                TypeCreature::Bone => inventory.add_bone(1),
                                TypeCreature::Arm => inventory.add_arms(1),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_game_state_on_inventory(
    query_inventory: Query<&Inventory>,
    mut app_state: ResMut<State<SceneModelState>>,
) {
    if let Ok(inventory) = query_inventory.get_single() {
        match app_state.current() {
            SceneModelState::FullBody => {}
            SceneModelState::HalfBody => {
                if inventory.has_enough_to_upgrade(TypeCreature::SkellyFullBody) {
                    app_state.set(SceneModelState::FullBody).unwrap();
                }
            }
            SceneModelState::OnlyHead => {
                if inventory.has_enough_to_upgrade(TypeCreature::SkellyHalf) {
                    app_state.set(SceneModelState::HalfBody).unwrap();
                }
            }
        }
    }
}
