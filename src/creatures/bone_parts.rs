use crate::animations_handler::{spawn_animation_stop_watch, VecSceneHandle};
use crate::creatures::{
    BoneTag, Creature, CurrentAnimationIndex, TypeCreature, GLTF_PATH_ARM, GLTF_PATH_BONE,
    GLTF_PATH_CHEST, GLTF_PATH_HEAD, GLTF_PATH_LEG,
};
use crate::map::{I_SHIFT, J_SHIFT};
use crate::{directions, AddAnimation, HashMapAnimationClip, SceneHandle, SkellyAnimationId};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

static GLTF_PATH_PACK_BONES: &str = "models/stack_bones/low_poly_bone_pile.glb#Scene0";

pub struct BonePlugin;
impl Plugin for BonePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_asset_parts)
            .add_startup_system(spawn_pack_bones)
            .add_system_to_stage(CoreStage::PostUpdate, update_sensor_bonepack);
    }
}

#[derive(Component)]
struct TagPackBone;

#[derive(Debug, Component)]
struct BonePack {
    consumed: bool,
    position: Vec3,
    items: Vec<TypeCreature>,
}

fn spawn_pack_bones(asset_server: Res<AssetServer>, mut commands: Commands) {
    let pack_handle = asset_server.load(GLTF_PATH_PACK_BONES);

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Chest,
            TypeCreature::Leg,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        9.0,
        7.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        11.0,
        8.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![TypeCreature::Bone, TypeCreature::Leg],
        11.0,
        7.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![TypeCreature::Bone, TypeCreature::Bone, TypeCreature::Bone],
        13.0,
        7.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        12.0,
        4.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        12.0,
        5.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        13.0,
        4.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        13.0,
        5.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![TypeCreature::Bone, TypeCreature::Bone],
        11.0,
        11.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Arm,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        14.0,
        12.0,
        pack_handle.clone(),
    );

    generate_one_pack(
        commands.borrow_mut(),
        vec![
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Arm,
            TypeCreature::Bone,
            TypeCreature::Bone,
            TypeCreature::Bone,
        ],
        14.0,
        3.0,
        pack_handle,
    );
}

fn generate_one_pack(
    commands: &mut Commands,
    items: Vec<TypeCreature>,
    i: f32,
    j: f32,
    handle: Handle<Scene>,
) {
    let position = i * I_SHIFT + j * J_SHIFT;

    let bonepack = BonePack {
        consumed: false,
        position,
        items,
    };

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(position.x, 0.0, position.z),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Default::default(),
                    scale: Vec3::ONE * 0.005 * bonepack.items.len() as f32,
                },
                ..default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform {
                        translation: Vec3::new(-0.6, 1.0, -0.5),
                        rotation: Default::default(),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Collider::cone(0.05, 1.0))
                .insert(Sensor)
            //.insert(Sensor)
            ;
        })
        .insert(TagPackBone)
        .insert(bonepack);
}

fn update_sensor_bonepack(
    parent_query: Query<&Parent>,
    mut query_bone: Query<&mut BonePack, With<TagPackBone>>,
    mut command: Commands,
    mut collision_events: EventReader<CollisionEvent>,

    vec_scene_handlers: Res<VecSceneHandle>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(child_a, child_b, _) = collision_event {
            for entity in [child_a, child_b] {
                if let Ok(parent) = parent_query.get(*entity) {
                    if let Ok(mut bonepack) = query_bone.get_mut(parent.get()) {
                        spawn_parts_from_pack(
                            command.borrow_mut(),
                            &mut bonepack,
                            &vec_scene_handlers,
                        );
                    }
                }
            }
        }
    }
}

fn spawn_parts_from_pack(
    commands: &mut Commands,
    mut bonepack: &mut BonePack,
    vec_scene_handlers: &Res<VecSceneHandle>,
) {
    if bonepack.consumed {
        return;
    }

    let pack_position = bonepack.position;

    let relative_positon = [J_SHIFT / 2.0, I_SHIFT / 2.0, -I_SHIFT / 2.0, -J_SHIFT / 2.0];

    let mut index = 0;

    for creature in &bonepack.items {
        spawn_part(
            commands,
            vec_scene_handlers,
            pack_position + relative_positon[index],
            *creature,
        );
        index = (index + 1) % 4;
    }

    bonepack.consumed = true;
}

/// Loads assets
fn load_asset_parts(
    //mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<AddAnimation>,
) {
    helper_load_asset(
        &asset_server,
        event_writer.borrow_mut(),
        GLTF_PATH_BONE,
        TypeCreature::Bone,
    );
    helper_load_asset(
        &asset_server,
        event_writer.borrow_mut(),
        GLTF_PATH_HEAD,
        TypeCreature::Head,
    );
    helper_load_asset(
        &asset_server,
        event_writer.borrow_mut(),
        GLTF_PATH_CHEST,
        TypeCreature::Chest,
    );
    helper_load_asset(
        &asset_server,
        event_writer.borrow_mut(),
        GLTF_PATH_LEG,
        TypeCreature::Leg,
    );
    helper_load_asset(
        &asset_server,
        event_writer.borrow_mut(),
        GLTF_PATH_ARM,
        TypeCreature::Arm,
    );
}

/// Old function used to make bone parts rotates
/// Unecessary with rapier::Velocity!
/*fn float_and_rotate_parts(mut query_parts: Query<&mut Transform, With<BoneTag>>) {
    for mut part_transform in query_parts.iter_mut() {
        let shift_rotation = (part_transform.rotation.to_axis_angle().1 + 0.05);
        info!("{} {} {}", part_transform.rotation, (shift_rotation), Quat::from_rotation_y(shift_rotation));
        part_transform.rotation = Quat::from_rotation_y(shift_rotation);
    }
}*/

/// Helper to load asset
fn helper_load_asset(
    asset_server: &Res<AssetServer>,
    event_writer: &mut EventWriter<AddAnimation>,
    scene_path: &str,
    type_creature: TypeCreature,
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
        type_creature,
        activated: true,
    };

    event_writer.send(AddAnimation {
        scene_handler: scene,
        target: None,
        start_animation: false,
    });
}

/// Spawn the part with Commands and create a stopwatch
fn spawn_part(
    commands: &mut Commands,
    vec_scene_handlers: &Res<VecSceneHandle>,
    position: Vec3,
    type_creature: TypeCreature,
) {
    for scene_handlers in &vec_scene_handlers.0 {
        if scene_handlers.type_creature == type_creature {
            // Adjusting the loaded scene
            let adjust_scale = Vec3::ONE * 1.25;
            let adjusted_transform = match type_creature {
                TypeCreature::Chest => Transform {
                    translation: Vec3::new(0.0, 0.0, 0.7),
                    rotation: Quat::from_scaled_axis(Vec3::new(-1.0, 0.0, 0.0)),
                    scale: adjust_scale,
                },
                TypeCreature::Head => Transform {
                    translation: Vec3::new(0.0, 0.0, -0.6),
                    rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, -0.3)),
                    scale: adjust_scale,
                },
                TypeCreature::Leg => Transform {
                    translation: Vec3::new(-0.4, 0.0, 0.0),
                    rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, -0.3)),
                    scale: adjust_scale,
                },
                TypeCreature::Bone => Transform {
                    translation: Vec3::new(-0.4, 0.0, -0.2),
                    rotation: Quat::from_scaled_axis(Vec3::new(0.4, 0.0, -0.5)),
                    scale: adjust_scale,
                },
                TypeCreature::Arm => Transform {
                    translation: Vec3::new(0.9, 0.0, -0.4),
                    rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, 0.7)),
                    scale: adjust_scale,
                },
                _ => Transform::default(),
            };

            let entity_id = commands
                .spawn()
                .insert_bundle(PbrBundle {
                    transform: Transform {
                        translation: position,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(SceneBundle {
                        scene: scene_handlers.handle.clone(),
                        transform: adjusted_transform,
                        ..default()
                    });
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(PbrBundle {
                            transform: Transform::from_xyz(0.0, 1.0, 0.0),
                            ..default()
                        })
                        .insert(Collider::ball(0.25));
                })
                .insert(BoneTag)
                .insert(RigidBody::KinematicVelocityBased)
                .insert(Velocity {
                    linvel: Default::default(),
                    angvel: Vec3::new(0.0, 1.0, 0.0),
                })
                .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
                .insert(Creature {
                    type_creature,
                    direction: directions::Direction::Up,
                    direction_vec3: Default::default(),
                    current_animation_index: CurrentAnimationIndex(0),
                    can_move: false,
                })
                .insert(Name::new(format!("{:#?}", type_creature)))
                .id();

            spawn_animation_stop_watch(entity_id.id(), 0, commands.borrow_mut());
        }
    }
}
