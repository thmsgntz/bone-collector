use crate::inventory::{
    ItemType, STARTING_NB_ARM, STARTING_NB_BONES, STARTING_NB_CHEST, STARTING_NB_LEG, TEXT_INV_ARM,
    TEXT_INV_BONE, TEXT_INV_CHEST, TEXT_INV_LEG,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryTextTag(pub ItemType);

/// One NodeBundle for the whole rectangle:
///  - One direct child is a inventory box:
///    - One child holding the image for the black bordered image (with alpha background)
///    - One child holding the image of the bone (with alpha background)
///    - One child for the text with one section tagged to be updated
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_arm_image = asset_server.load("models/arm/arm.png");
    let handle_chest_image = asset_server.load("models/chest/chest.png");
    let handle_bone_image = asset_server.load("models/bone/bone.png");
    let handle_leg_image = asset_server.load("models/leg/leg.png");
    let handle_cadre = asset_server.load("cadre.png");
    let handle_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Bones
    let (first_cadre, first_bone, first_text) = children_node_ui(
        handle_cadre.clone(),
        handle_bone_image,
        handle_font.clone(),
        format!("{} {}", TEXT_INV_BONE, STARTING_NB_BONES),
    );

    // Arms
    let (second_cadre, second_bone, second_text) = children_node_ui(
        handle_cadre.clone(),
        handle_arm_image,
        handle_font.clone(),
        format!("{} {}", TEXT_INV_ARM, STARTING_NB_ARM),
    );

    // Legs
    let (third_cadre, third_bone, third_text) = children_node_ui(
        handle_cadre.clone(),
        handle_leg_image,
        handle_font.clone(),
        format!("{} {}", TEXT_INV_LEG, STARTING_NB_LEG),
    );

    // Chest
    let (fourth_cadre, fourth_bone, fourth_text) = children_node_ui(
        handle_cadre,
        handle_chest_image,
        handle_font,
        format!("{} {}", TEXT_INV_CHEST, STARTING_NB_CHEST),
    );

    commands
        .spawn_bundle(NodeBundle {
            node: Default::default(),
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::SpaceEvenly,
                position: UiRect::new(
                    Val::Percent(25.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(1.0),
                ),
                ..default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.5)), // Todo : alpha = 255 quand terminé
            ..default()
        })
        /* FIRST CADRE BONES */
        .with_children(|parent| {
            parent
                .spawn_bundle(first_cadre)
                .with_children(|parent| {
                    parent.spawn_bundle(first_bone);
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(first_text)
                        .insert(InventoryTextTag(ItemType::Bone));
                });
        })
        /* SECOND CADRE ARMS */
        .with_children(|parent| {
            parent
                .spawn_bundle(second_cadre)
                .with_children(|parent| {
                    parent.spawn_bundle(second_bone);
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(second_text)
                        .insert(InventoryTextTag(ItemType::Arm));
                });
        })
        /* THIRD CADRE LEGS */
        .with_children(|parent| {
            parent
                .spawn_bundle(third_cadre)
                .with_children(|parent| {
                    parent.spawn_bundle(third_bone);
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(third_text)
                        .insert(InventoryTextTag(ItemType::Leg));
                });
        })
        /* FOURTH CADRE CHEST */
        .with_children(|parent| {
            parent
                .spawn_bundle(fourth_cadre)
                .with_children(|parent| {
                    parent.spawn_bundle(fourth_bone);
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(fourth_text)
                        .insert(InventoryTextTag(ItemType::Chest));
                });
        });
}

/// A lot of tries and retries using egui to obtain good results
fn children_node_ui(
    handle_cadre: Handle<Image>,
    handle_image: Handle<Image>,
    handle_font: Handle<Font>,
    text_value: String,
) -> (NodeBundle, NodeBundle, TextBundle) {
    let image_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexEnd,
            aspect_ratio: Some(1.0),
            ..default()
        },
        color: Color::WHITE.into(),
        image: UiImage(handle_cadre),

        ..default()
    };

    let bone_node = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexEnd,
            aspect_ratio: Some(1.0),
            ..default()
        },
        color: Color::rgb(1.0, 1.0, 1.0).into(),
        image: UiImage(handle_image),
        ..default()
    };

    let text_node = TextBundle::from_section(
        text_value,
        TextStyle {
            font: handle_font,
            font_size: 15.0,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        align_self: AlignSelf::FlexStart,
        position: UiRect::new(
            Val::Percent(37.0),
            Val::Percent(0.0),
            Val::Percent(0.0),
            Val::Percent(5.0),
        ),
        ..default()
    });

    (image_node, bone_node, text_node)
}
