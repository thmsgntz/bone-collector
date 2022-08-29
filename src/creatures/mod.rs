use crate::animations_handler::{
    AddAnimation, AnimationEntityLink, AnimationStopWatch, ChangeAnimation, TagPlayerScene,
};
use crate::creatures::skelly::{Skelly, SkellyAnimationId};
use bevy::math::vec3;

use crate::camera::ShiftFromPlayer;
use crate::creatures::SceneModelState::{FullBody, HalfBody, OnlyHead};
use crate::map::{I_SHIFT, J_SHIFT};
use crate::{directions, SceneHandle};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

mod bone_parts;
pub(crate) mod skelly;

/// marker
#[derive(Component)]
pub(crate) struct BoneTag;

pub static GLTF_PATH_FULL_BODY: &str = "models/full_body/scene.gltf";
pub static GLTF_PATH_HALF_BODY: &str = "models/half/half_body.gltf";
pub static GLTF_PATH_CHEST: &str = "models/chest/chest.gltf";
pub static GLTF_PATH_HEAD: &str = "models/head/head_with_animation.gltf";
pub static GLTF_PATH_LEG: &str = "models/leg/leg.gltf";
pub static GLTF_PATH_BONE: &str = "models/bone/bone.gltf";
pub static GLTF_PATH_ARM: &str = "models/arm/arm.gltf";

pub const BONES_NEEDED_HALF_BODY: usize = 10;
pub const CHEST_NEEDED_HALF_BODY: usize = 1;
pub const LEGS_NEEDED_HALF_BODY: usize = 2;

pub const BONES_NEEDED_FULL_BODY: usize = 45;
pub const CHEST_NEEDED_FULL_BODY: usize = 1;
pub const ARMS_NEEDED_FULL_BODY: usize = 2;
pub const LEGS_NEEDED_FULL_BODY: usize = 2;

pub trait CreatureTrait {
    fn spawn(
        commands: Commands,
        asset_server: Res<AssetServer>,
        event_writer: EventWriter<AddAnimation>,
    );

    fn update_animation(
        target: u32,
        index_animation: usize,
        event_writer: &mut EventWriter<ChangeAnimation>,
    );

    fn can_move(animation_index: usize) -> bool;
}

#[derive(Component)]
pub struct ToDespawn;

/// Player marker
#[derive(Component)]
pub(crate) struct Player;

/// Vec containing pointers to:
///   - Scene with full body skeleton
///   - Scene with body without arms
///   - Scene with only heads
pub struct VecSkellyScenes(pub Vec<SceneHandle>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneModelState {
    FullBody,
    HalfBody,
    OnlyHead,
}

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bone_parts::BonePlugin)
            .add_state(OnlyHead)
            .add_system_set(SystemSet::on_exit(OnlyHead).with_system(update_player_model))
            .add_system_set(SystemSet::on_exit(FullBody).with_system(update_player_model))
            .add_system_set(SystemSet::on_exit(HalfBody).with_system(update_player_model))
            .add_startup_system(spawn_skelly)
            .add_system(keyboard_control)
            .add_system_to_stage(CoreStage::First, check_falling_player)
            .add_system(cleanup_creature);
    }
}

fn spawn_skelly(
    command: Commands,
    asset_server: Res<AssetServer>,
    event_writer: EventWriter<AddAnimation>,
) {
    Skelly::spawn(command, asset_server, event_writer);
}

//#[derive(Bundle, Clone)]

/// Contient l'index de l'animation en cours
/// Mis à jour par animations_handler:update_animation
#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub struct CurrentAnimationIndex(pub usize);

impl PartialEq<SkellyAnimationId> for CurrentAnimationIndex {
    fn eq(&self, other: &SkellyAnimationId) -> bool {
        self.get() == *other as usize
    }
}

impl From<usize> for CurrentAnimationIndex {
    fn from(a: usize) -> Self {
        Self(a)
    }
}

impl CurrentAnimationIndex {
    fn get(&self) -> usize {
        self.0
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum TypeCreature {
    SkellyFullBody,
    SkellyOnlyHead,
    SkellyHalf,
    Chest,
    Head,
    Leg,
    Bone,
    Arm,
}

//#[derive(Bundle)]
#[derive(Component)]
pub struct Creature {
    pub type_creature: TypeCreature,
    pub direction: directions::Direction,
    pub direction_vec3: Vec3,
    /// index (in vec_animations)  of current animation being played
    pub current_animation_index: CurrentAnimationIndex,
    pub can_move: bool,
}

impl Creature {
    pub fn update_animation(
        &self,
        target: u32,
        index_animation: usize,
        event_writer: &mut EventWriter<ChangeAnimation>,
    ) {
        match self.type_creature {
            TypeCreature::SkellyFullBody
            | TypeCreature::SkellyHalf
            | TypeCreature::SkellyOnlyHead => {
                Skelly::update_animation(target, index_animation, event_writer);
            }
            _ => {
                event_writer.send(ChangeAnimation {
                    target,
                    index: 0,
                    repeat: true,
                });
            }
        }
    }
}

fn check_falling_player(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    shift_value: Res<ShiftFromPlayer>,
    mut query_camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok((mut player_transform, mut velocity)) = player_query.get_single_mut() {
        if player_transform.translation.y < -2.0 {
            info!("Falling");
            let starting_position = 7.0 * I_SHIFT + 7.0 * J_SHIFT;
            player_transform.translation = Vec3::new(starting_position.x, 2.0, starting_position.z);
            velocity.linvel = Vec3::ZERO;
            if let Ok(mut camera_transform) = query_camera.get_single_mut() {
                let shift = shift_value.0;
                *camera_transform = Transform::from_xyz(
                    player_transform.translation.x - shift,
                    camera_transform.translation.y,
                    player_transform.translation.z - shift,
                )
                .looking_at(player_transform.translation, Vec3::Y);
            }
        }
    }
}

fn send_new_animation(
    target_entity: u32,
    animation_index: usize,
    do_repeat: bool,
    mut event_writer: EventWriter<ChangeAnimation>,
) {
    event_writer.send(ChangeAnimation {
        target: target_entity,
        index: animation_index,
        repeat: do_repeat,
    });
}

fn keyboard_control(
    event_writer: EventWriter<ChangeAnimation>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut query_player: Query<(Entity, &mut Transform, &mut Velocity, &mut Creature), With<Player>>,
) {
    let mut vector_direction = Vec3::ZERO;
    let mut is_shift = 0.0;

    if keyboard_input.pressed(KeyCode::Z) || keyboard_input.pressed(KeyCode::W) {
        vector_direction += Vec3::new(1.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        vector_direction += Vec3::new(-1.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        vector_direction += Vec3::new(-1.0, 0.0, -1.0);
    }

    if keyboard_input.pressed(KeyCode::Q) || keyboard_input.pressed(KeyCode::A) {
        vector_direction += Vec3::new(1.0, 0.0, -1.0);
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        is_shift = 1.0;
    }

    if let Ok((entity, mut player_transform, mut player_velocity, mut player_creature)) =
        query_player.get_single_mut()
    {
        if player_transform.translation.y < -2.0 || player_transform.translation.y > 1.0 {
            keyboard_input.reset_all();
            return;
        }

        if player_creature.type_creature == TypeCreature::SkellyOnlyHead {
            is_shift = 0.0;
        }

        let idle_index = SkellyAnimationId::Idle as usize;

        // Returns if vector_direction is 0
        if vector_direction == Vec3::ZERO {
            if player_creature.current_animation_index == SkellyAnimationId::Walk
                || player_creature.current_animation_index == SkellyAnimationId::Run
            {
                send_new_animation(entity.id(), idle_index, true, event_writer);
            }

            player_velocity.linvel = vec3(0.0, player_velocity.linvel.y, 0.0);
            return;
        }

        // Returns if the player can not move
        match player_creature.type_creature {
            TypeCreature::SkellyFullBody | TypeCreature::SkellyHalf => {
                if !Skelly::can_move(player_creature.current_animation_index.0) {
                    return;
                }
            }
            TypeCreature::SkellyOnlyHead => {}
            _ => return,
        }

        // Update Transform.translation
        let mut translation = player_creature.direction_vec3.lerp(vector_direction, 0.1);
        player_creature.direction_vec3 = translation;
        translation.y = player_velocity.linvel.y;

        player_velocity.linvel = translation * 2.0 * (1.0 + (is_shift * 2.0));

        // Update rotation
        let direction = directions::map_vec3_to_direction(vector_direction).unwrap();
        let qu = Quat::from_rotation_y(direction.get_angle());
        //let rotation = if player_transform.rotation.angle_between(qu).abs() > 3.0 {
        //    qu
        //} else {
        //    player_transform.rotation.lerp(qu, 0.1)
        //};
        let rotation = player_transform.rotation.lerp(qu, 0.1);
        player_transform.rotation = rotation;

        // no running animation when only head
        if player_creature.type_creature == TypeCreature::SkellyOnlyHead {
            return;
        }

        let moving_animation = if is_shift >= 1.0 {
            SkellyAnimationId::Run as usize
        } else {
            SkellyAnimationId::Walk as usize
        };

        // no need to update animation
        if moving_animation as usize == player_creature.current_animation_index.0 {
            return;
        }

        // do we need to update animation? depends it was already walking/running and if pressing shift
        // I think this can be easily less complicated, but no time
        if (player_creature.current_animation_index.0 != SkellyAnimationId::Run as usize
            && player_creature.current_animation_index.0 != SkellyAnimationId::Walk as usize)
            || (player_creature.current_animation_index.0 == SkellyAnimationId::Run as usize
                && moving_animation == SkellyAnimationId::Walk as usize)
            || (player_creature.current_animation_index.0 == SkellyAnimationId::Walk as usize
                && moving_animation == SkellyAnimationId::Run as usize)
        {
            send_new_animation(entity.id(), moving_animation, true, event_writer);
        }
    }
}

fn update_player_model(
    mut command: Commands,
    vec_scenes: Res<VecSkellyScenes>,
    scene_state: Res<State<SceneModelState>>,
    mut query_child_scene: Query<Entity, With<TagPlayerScene>>,
    mut query_player: Query<(Entity, &AnimationEntityLink, &mut Creature), With<Player>>,
    mut query_stopwatch: Query<&mut AnimationStopWatch>,
) {
    if let Ok(child_scene) = query_child_scene.get_single_mut() {
        if let Ok((player_entity, animation_player, mut creature)) = query_player.get_single_mut() {
            info!("Child found {:?}", child_scene);

            // remove current:,
            info!("Skelly : {:?}", player_entity);
            info!("Despawning: {:?} / {:?}", child_scene, animation_player.0);
            command
                .entity(player_entity)
                .remove_children(&[child_scene]);
            command.entity(child_scene).despawn_recursive();
            command.entity(animation_player.0).despawn_recursive();
            command
                .entity(player_entity)
                .remove::<AnimationEntityLink>();

            info!("Calling update on_exit: {:?}", *scene_state.current());

            let scene_full_body = &vec_scenes.0[0];
            let scene_half = &vec_scenes.0[1];
            let scene_head = &vec_scenes.0[2];

            let mut index_animation = SkellyAnimationId::None as usize;

            match scene_state.current() {
                OnlyHead => {
                    // Désactive HEAD, active HALF

                    creature.type_creature = TypeCreature::SkellyHalf;

                    // add new
                    command.entity(player_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(SceneBundle {
                                scene: scene_half.handle.clone(),
                                transform: Transform {
                                    translation: Default::default(),
                                    rotation: Default::default(),
                                    scale: Vec3::ONE * 0.6,
                                },
                                ..default()
                            })
                            .insert(TagPlayerScene);
                    });
                }
                FullBody => {
                    // Désactive FULL_BODY, active HEAD
                    creature.type_creature = TypeCreature::SkellyOnlyHead;

                    index_animation = SkellyAnimationId::Idle as usize;

                    // add new
                    command.entity(player_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(SceneBundle {
                                scene: scene_head.handle.clone(),
                                transform: Transform {
                                    translation: Vec3::new(0.0, -0.5, 0.0),
                                    rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, 0.0)),
                                    scale: Vec3::ONE * 0.6,
                                },
                                ..default()
                            })
                            .insert(TagPlayerScene);
                    });
                }
                HalfBody => {
                    // DESACTIVATE HALF, ACTIVATE FULL
                    creature.type_creature = TypeCreature::SkellyFullBody;

                    // add new
                    command.entity(player_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(SceneBundle {
                                scene: scene_full_body.handle.clone(),
                                transform: Transform {
                                    translation: Default::default(),
                                    rotation: Default::default(),
                                    scale: Vec3::ONE * 0.6,
                                },
                                ..default()
                            })
                            .insert(TagPlayerScene);
                    });
                }
            }
            creature.current_animation_index.0 = index_animation;
            for mut stopwatch in query_stopwatch.iter_mut() {
                if stopwatch.creature_entity_id == player_entity.id() {
                    stopwatch.index_animation = index_animation;
                    stopwatch.manual_termination = true;
                }
            }
        }
    }
}

fn cleanup_creature(
    mut commands: Commands,
    q: Query<Entity, With<ToDespawn>>,
    query_stopwatch: Query<(Entity, &AnimationStopWatch)>,
) {
    for e in q.iter() {
        // remove stopwatch
        for (e_sw, stopwatch) in query_stopwatch.iter() {
            if stopwatch.creature_entity_id == e.id() {
                commands.entity(e_sw).despawn_recursive();
            }
        }

        // remove creature
        commands.entity(e).despawn_recursive();
    }
}
