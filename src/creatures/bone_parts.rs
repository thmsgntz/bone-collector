use std::borrow::BorrowMut;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::{AddAnimation, HashMapAnimationClip, SceneHandle, SkellyAnimationId};
use crate::animations_handler::{spawn_animation_stop_watch, VecSceneHandle};
use crate::creatures::{GLTF_PATH_BONE, GLTF_PATH_CHEST, GLTF_PATH_HEAD, GLTF_PATH_LEG, TypeCreature};

pub struct BonePlugin;
impl Plugin for BonePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_asset_parts)
            .add_system(spawn_bone_part);

    }
}

/// marker
#[derive(Component)]
struct BoneTag;

/// Loads assets
fn load_asset_parts(
    //mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<AddAnimation>,
) {
    helper_load_asset(&asset_server, event_writer.borrow_mut(), GLTF_PATH_BONE, TypeCreature::Bone);
    helper_load_asset(&asset_server, event_writer.borrow_mut(), GLTF_PATH_HEAD, TypeCreature::Head);
    helper_load_asset(&asset_server, event_writer.borrow_mut(), GLTF_PATH_CHEST, TypeCreature::Chest);
    helper_load_asset(&asset_server, event_writer.borrow_mut(), GLTF_PATH_LEG, TypeCreature::Leg);
}

/// Helper to load asset
fn helper_load_asset(
    asset_server: &Res<AssetServer>,
    event_writer: &mut EventWriter<AddAnimation>,
    scene_path: &str,
    type_creature: TypeCreature
) {
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let mut hm_animations = HashMapAnimationClip::new();

    let id = 0;
    let handle = asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str());
    hm_animations.insert(id as usize, SkellyAnimationId::Idle.get_duration(), handle);

    let scene = SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
        creature_entity_id: None,
        type_creature
    };

    /*let bone_id = commands.spawn()
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: position,
                rotation: Default::default(),
                scale: Vec3::ONE,
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: scene.handle.clone(),
                transform: Transform {
                    translation: Default::default(),
                    rotation: Default::default(),
                    scale: Vec3::ONE,
                },
                ..default()
            });
        }).insert(BoneTag)
        .id();*/

    event_writer.send(AddAnimation {
        scene_handler: scene,
        target: None,
        start_animation: false
    });
}

fn spawn_bone_part(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    vec_scene_handlers: Res<VecSceneHandle>,
) {
    if keyboard_input.pressed(KeyCode::B) {
        // B for bone
        for scene_handlers in &vec_scene_handlers.0 {
            if scene_handlers.type_creature == TypeCreature::Bone {
                info!("Spawning {:#?}", scene_handlers.type_creature);
                let entity_id = commands.spawn()
                    .insert_bundle(PbrBundle {
                        transform: Transform {
                            translation: Vec3::new(-1.0, 0.0, -1.0),
                            rotation: Default::default(),
                            scale: Vec3::ONE,
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(SceneBundle {
                            scene: scene_handlers.handle.clone(),
                            transform: Transform {
                                translation: Default::default(),
                                rotation: Default::default(),
                                scale: Vec3::ONE * 6.0, // TODO ici :)
                            },
                            ..default()
                        });
                    }).insert(BoneTag).id()
                    ;

                    spawn_animation_stop_watch(
                        entity_id.id(),
                        0,
                        commands.borrow_mut()
                    );
                keyboard_input.reset_all();
            }
        }
    }
}