use crate::animations_handler::{spawn_animation_stop_watch, VecSceneHandle};
use crate::creatures::{
    Creature, CurrentAnimationIndex, TypeCreature, GLTF_PATH_ARM, GLTF_PATH_BONE, GLTF_PATH_CHEST,
    GLTF_PATH_HEAD, GLTF_PATH_LEG,
};
use crate::{directions, AddAnimation, HashMapAnimationClip, SceneHandle, SkellyAnimationId};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use std::borrow::BorrowMut;
use std::f32::consts::PI;

pub struct BonePlugin;
impl Plugin for BonePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_asset_parts)
            .add_system(keyboard_spawn_bone_part)
            .add_system(float_and_rotate_parts);
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

fn float_and_rotate_parts(mut query_parts: Query<&mut Transform, With<BoneTag>>) {
    for mut part_transform in query_parts.iter_mut() {
        let shift_rotation = (part_transform.rotation.to_axis_angle().1 + 0.1) % (2.0 * PI);
        part_transform.rotation = Quat::from_rotation_y(shift_rotation);

        //todo : add translation y ^ v
    }
}

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
    };

    event_writer.send(AddAnimation {
        scene_handler: scene,
        target: None,
        start_animation: false,
    });
}

fn keyboard_spawn_bone_part(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    vec_scene_handlers: Res<VecSceneHandle>,
) {
    if keyboard_input.pressed(KeyCode::B) {
        // B for bone
        spawn_part(
            commands.borrow_mut(),
            &vec_scene_handlers,
            Vec3::new(2.5, 0.0, 2.5),
            TypeCreature::Bone,
        );

        spawn_part(
            commands.borrow_mut(),
            &vec_scene_handlers,
            Vec3::new(-2.5, 0.0, -2.5),
            TypeCreature::Leg,
        );

        spawn_part(
            commands.borrow_mut(),
            &vec_scene_handlers,
            Vec3::new(-2.5, 0.0, 2.5),
            TypeCreature::Arm,
        );

        spawn_part(
            commands.borrow_mut(),
            &vec_scene_handlers,
            Vec3::new(2.5, 0.0, -2.5),
            TypeCreature::Chest,
        );

        keyboard_input.reset_all();
    }
}

/// Spawn the part with Commands and create a stopwatch
fn spawn_part(
    mut commands: &mut Commands,
    vec_scene_handlers: &Res<VecSceneHandle>,
    position: Vec3,
    type_creature: TypeCreature,
) {
    for scene_handlers in &vec_scene_handlers.0 {
        if scene_handlers.type_creature == type_creature {
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
                        transform: Transform {
                            scale: Vec3::ONE,
                            ..default()
                        },
                        ..default()
                    });
                })
                .insert(BoneTag)
                .insert(Creature {
                    type_creature: type_creature.clone(),
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
