use crate::animations_handler::{
    AddAnimation, AnimationEntityLink, AnimationStopWatch, ChangeAnimation, TagPlayerScene,
};
use crate::creatures::skelly::{Skelly, SkellyAnimationId};
use bevy::math::vec3;

use crate::creatures::SceneModelState::{FullBody, OnlyHead};
use crate::{directions, SceneHandle};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

mod bone_parts;
pub(crate) mod skelly;

/// marker
#[derive(Component)]
pub(crate) struct BoneTag;

// const ENTITY_SPEED: f32 = 2.0;
// const ENTITY_SPEED_ROTATION: f32 = 0.1;

pub static GLTF_PATH_FULL_BODY: &str = "models/full_body/scene.gltf";
pub static GLTF_PATH_CHEST: &str = "models/chest/chest.gltf";
pub static GLTF_PATH_HEAD: &str = "models/head/head.gltf";
pub static GLTF_PATH_LEG: &str = "models/leg/leg.gltf";
pub static GLTF_PATH_BONE: &str = "models/bone/bone.gltf";
pub static GLTF_PATH_ARM: &str = "models/arm/arm.gltf";

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

/// Player marker
pub(crate) struct SceneFullBody(SceneHandle);

/// Player marker
pub(crate) struct SceneOnlyHead(SceneHandle);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum SceneModelState {
    FullBody,
    OnlyHead,
}

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bone_parts::BonePlugin)
            .add_state(FullBody)
            .add_system_set(SystemSet::on_exit(OnlyHead).with_system(update_player_model))
            .add_system_set(SystemSet::on_exit(FullBody).with_system(update_player_model))
            .add_startup_system(spawn_skelly)
            .add_system(keyboard_control)
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

#[derive(PartialEq, Eq, Clone, Debug)]
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
            TypeCreature::SkellyFullBody => {
                Skelly::update_animation(target, index_animation, event_writer);
            }
            _ => {
                info!("Sending Idle from update_animation");
                event_writer.send(ChangeAnimation {
                    target,
                    index: 0,
                    repeat: true,
                });
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
    keyboard_input: Res<Input<KeyCode>>,
    mut query_player: Query<(Entity, &mut Transform, &mut Velocity, &mut Creature), With<Player>>,
) {
    let mut vector_direction = Vec3::ZERO;
    let mut is_shift = 0.0;

    if keyboard_input.pressed(KeyCode::Z) {
        vector_direction += Vec3::new(1.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        vector_direction += Vec3::new(-1.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        vector_direction += Vec3::new(-1.0, 0.0, -1.0);
    }

    if keyboard_input.pressed(KeyCode::Q) {
        vector_direction += Vec3::new(1.0, 0.0, -1.0);
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        is_shift = 1.0;
    }

    if let Ok((entity, mut player_transform, mut player_velocity, mut player_creature)) =
        query_player.get_single_mut()
    {
        let idle_index = match player_creature.type_creature {
            TypeCreature::SkellyFullBody => SkellyAnimationId::Idle as usize,
            _ => 0,
        };

        // Returns if vector_direction is 0
        if vector_direction == Vec3::ZERO {
            if player_creature.current_animation_index == SkellyAnimationId::Walk {
                send_new_animation(entity.id(), idle_index, true, event_writer);
            }

            player_velocity.linvel = vec3(0.0, player_velocity.linvel.y, 0.0);
            return;
        }

        // Returns if the player can not move
        match player_creature.type_creature {
            TypeCreature::SkellyFullBody => {
                if !Skelly::can_move(player_creature.current_animation_index.0) {
                    return;
                }
            }
            TypeCreature::SkellyOnlyHead | TypeCreature::SkellyHalf => {}
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
        let rotation = if player_transform.rotation.angle_between(qu).abs() > 3.0 {
            qu
        } else {
            player_transform.rotation.lerp(qu, 0.1)
        };
        player_transform.rotation = rotation;

        if player_creature.current_animation_index.0 != SkellyAnimationId::Walk as usize
            && player_creature.type_creature == TypeCreature::SkellyFullBody
        {
            send_new_animation(
                entity.id(),
                SkellyAnimationId::Walk as usize,
                true,
                event_writer,
            );
        }
    }
}

fn update_player_model(
    mut command: Commands,
    mut scene_full_body: ResMut<SceneFullBody>,
    mut scene_head: ResMut<SceneOnlyHead>,
    scene_state: Res<State<SceneModelState>>,
    mut query_child_scene: Query<Entity, With<TagPlayerScene>>,
    mut query_player: Query<(Entity, &AnimationEntityLink, &mut Creature), With<Player>>,
    mut query_stopwatch: Query<&mut AnimationStopWatch>,
) {
    info!("Bonjour");

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

            match scene_state.current() {
                OnlyHead => {
                    // Désactive HEAD, active FULL_BODY
                    scene_head.0.activated = false;
                    scene_full_body.0.activated = true;

                    creature.type_creature = TypeCreature::SkellyFullBody;
                    creature.can_move = true;

                    // add new
                    command.entity(player_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(SceneBundle {
                                scene: scene_full_body.0.handle.clone(),
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
                    scene_full_body.0.activated = false;
                    scene_head.0.activated = true;

                    creature.type_creature = TypeCreature::SkellyOnlyHead;
                    creature.can_move = true;

                    // add new
                    command.entity(player_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(SceneBundle {
                                scene: scene_head.0.handle.clone(),
                                transform: Transform {
                                    translation: Vec3::new(0.0, -1.2, 0.0),
                                    rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, 0.0)),
                                    scale: Vec3::ONE,
                                },
                                ..default()
                            })
                            .insert(TagPlayerScene);
                    });
                }
            }

            for mut stopwatch in query_stopwatch.iter_mut() {
                if stopwatch.creature_entity_id == player_entity.id() {
                    stopwatch.index_animation = 0;
                    stopwatch.manual_termination = true;
                    break;
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
