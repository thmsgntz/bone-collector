use crate::animations_handler::{AddAnimation, ChangeAnimation, HashMapAnimationClip, SceneHandle};
use crate::creatures::{Creature, CreatureTrait, CurrentAnimationIndex, GLTF_PATH_FULL_BODY, Player, TypeCreature};
use crate::directions;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum SkellyAnimationId {
    Spawn,         // ?
    Idle,          // duration: 1.5800002
    LookingAround, // duration: 3.1800003
    Attack,        // duration: 2.3200002
    Yell,          // duration: 1.5800002
    Walk,          // duration: 0.9800001
    Run,           // duration: 0.78000003
    Fall,          // ?
    Hit,           // ?
    Die,           // ?
    Hanged,        // ?
    None,          // ?
}

impl From<usize> for SkellyAnimationId {
    fn from(u: usize) -> Self {
        match u {
            0 => SkellyAnimationId::Spawn,
            1 => SkellyAnimationId::Idle,
            2 => SkellyAnimationId::LookingAround,
            3 => SkellyAnimationId::Attack,
            4 => SkellyAnimationId::Yell,
            5 => SkellyAnimationId::Walk,
            6 => SkellyAnimationId::Run,
            7 => SkellyAnimationId::Fall,
            8 => SkellyAnimationId::Hit,
            9 => SkellyAnimationId::Die,
            10 => SkellyAnimationId::Hanged,
            _ => SkellyAnimationId::None,
        }
    }
}

/*impl Into<usize> for SkellyAnimationId {
    fn into(self) -> usize {
        match self {
            SkellyAnimationId::Spawn => 0,
            SkellyAnimationId::Idle => 1,
            SkellyAnimationId::LookingAround => 2,
            SkellyAnimationId::Attack => 3,
            SkellyAnimationId::Yell => 4,
            SkellyAnimationId::Walk => 5,
            SkellyAnimationId::Run => 6,
            SkellyAnimationId::Fall => 7,
            SkellyAnimationId::Hit => 8,
            SkellyAnimationId::Die => 9,
            SkellyAnimationId::Hanged => 10,
            SkellyAnimationId::None => 11,
        }
    }
}*/

const SKELLY_ANIM_DURATION_SPAWN: f32 = 1.30;
const SKELLY_ANIM_DURATION_IDLE: f32 = 1.58;
const SKELLY_ANIM_DURATION_LOOKING_AROUND: f32 = 3.18;
const SKELLY_ANIM_DURATION_ATTACK: f32 = 2.32;
const SKELLY_ANIM_DURATION_YELL: f32 = 1.58;
const SKELLY_ANIM_DURATION_WALK: f32 = 0.98;
const SKELLY_ANIM_DURATION_RUN: f32 = 0.78;
const SKELLY_ANIM_DURATION_FALL: f32 = 1.1;
const SKELLY_ANIM_DURATION_HIT: f32 = 0.62;
const SKELLY_ANIM_DURATION_DIE: f32 = 1.06;
const SKELLY_ANIM_DURATION_HANGED: f32 = 1.58;

/*impl Into<CurrentAnimationIndex> for SkellyAnimationId {
    fn into(self) -> CurrentAnimationIndex {
        CurrentAnimationIndex(self)
    }
}*/

impl From<SkellyAnimationId> for CurrentAnimationIndex {
    fn from(id: SkellyAnimationId) -> Self {
        let usi = id as usize;
        CurrentAnimationIndex(usi)
    }
}

impl SkellyAnimationId {
    pub(crate) fn get_duration(&self) -> f32 {
        match self {
            SkellyAnimationId::Idle => SKELLY_ANIM_DURATION_IDLE,
            SkellyAnimationId::LookingAround => SKELLY_ANIM_DURATION_LOOKING_AROUND,
            SkellyAnimationId::Attack => SKELLY_ANIM_DURATION_ATTACK,
            SkellyAnimationId::Yell => SKELLY_ANIM_DURATION_YELL,
            SkellyAnimationId::Walk => SKELLY_ANIM_DURATION_WALK,
            SkellyAnimationId::Run => SKELLY_ANIM_DURATION_RUN,
            SkellyAnimationId::Fall => SKELLY_ANIM_DURATION_FALL,
            SkellyAnimationId::Hit => SKELLY_ANIM_DURATION_HIT,
            SkellyAnimationId::Die => SKELLY_ANIM_DURATION_DIE,
            SkellyAnimationId::Spawn => SKELLY_ANIM_DURATION_SPAWN,
            SkellyAnimationId::Hanged => SKELLY_ANIM_DURATION_HANGED,
            SkellyAnimationId::None => 0.0,
        }
    }
}

pub(crate) struct Skelly;
impl CreatureTrait for Skelly {
    fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut event_writer: EventWriter<AddAnimation>,
    ) {
        // let mut skelly_scene_handle = setup_skelly(&asset_server, "models/skeleton/scene.gltf");
        let mut skelly_scene_handle = setup_skelly(&asset_server, GLTF_PATH_FULL_BODY);

        // Skeleton
        let skelly_id = commands
            .spawn()
            .insert_bundle(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(directions::Direction::Up.get_angle()),
                    scale: Vec3::ONE,
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: skelly_scene_handle.handle.clone(),
                    transform: Transform {
                        translation: Default::default(),
                        rotation: Default::default(),
                        scale: Vec3::ONE * 0.6,
                    },
                    ..default()
                });
            })
            .with_children(|children| {
                children
                    .spawn()
                    .insert(Collider::cuboid(0.1, 0.9, 0.1))
                    .insert_bundle(PbrBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, 1.0, 0.0),
                            rotation: Quat::from_rotation_y(
                                directions::Direction::Left.get_angle(),
                            ), // Direction::Left
                            scale: Vec3::ONE,
                        },
                        ..Default::default()
                    });
            })
            .insert(RigidBody::Dynamic)
            .insert(
                LockedAxes::ROTATION_LOCKED_X
                    | LockedAxes::ROTATION_LOCKED_Z
                    | LockedAxes::ROTATION_LOCKED_Y,
            )
            .insert(Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert(Creature {
                type_creature: TypeCreature::Skelly,
                direction: directions::Direction::Up,
                direction_vec3: directions::Direction::Up.get_vec3(),
                current_animation_index: CurrentAnimationIndex::from(
                    SkellyAnimationId::Idle as usize,
                ),
                can_move: false,
            })
            .insert(Player)
            .insert(Name::new("Skelly"))
            .id();

        skelly_scene_handle.creature_entity_id = Some(skelly_id.id());

        event_writer.send(AddAnimation {
            scene_handler: skelly_scene_handle,
            target: Some(skelly_id.id()),
            start_animation: true
        });
    }

    fn update_animation(
        target: u32,
        index_animation: usize,
        event_writer: &mut EventWriter<ChangeAnimation>,
    ) {
        let mut new_animation = SkellyAnimationId::Idle;
        let mut repeat = false;

        match SkellyAnimationId::from(index_animation) {
            SkellyAnimationId::Idle => {}
            SkellyAnimationId::LookingAround => {
                new_animation = SkellyAnimationId::Idle;
                repeat = true;
            }
            SkellyAnimationId::Attack => {}
            SkellyAnimationId::Yell => {}
            SkellyAnimationId::Walk => {}
            SkellyAnimationId::Run => {}
            SkellyAnimationId::Fall => {}
            SkellyAnimationId::Hit => {}
            SkellyAnimationId::Die => {}
            SkellyAnimationId::Spawn => {
                new_animation = SkellyAnimationId::LookingAround;
                repeat = false;
            }
            SkellyAnimationId::Hanged => {}
            SkellyAnimationId::None => {}
        }

        event_writer.send(ChangeAnimation {
            target,
            index: new_animation as usize,
            repeat,
        });
    }

    /// Returns true is animation is Idle / Walk / Run
    fn can_move(animation_index: usize) -> bool {
        matches! {
            SkellyAnimationId::from(animation_index),
            SkellyAnimationId::Idle
            | SkellyAnimationId::Walk
            | SkellyAnimationId::Run
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_move() {
        let mut creature = Creature {
            type_creature: TypeCreature::Skelly,
            direction: directions::Direction::Up,
            direction_vec3: Default::default(),
            current_animation_index: CurrentAnimationIndex(SkellyAnimationId::Idle as usize),
            can_move: false,
        };

        assert_eq!(
            true,
            Skelly::can_move(creature.current_animation_index.0 as usize)
        );

        creature.current_animation_index = CurrentAnimationIndex(SkellyAnimationId::Spawn as usize);
        assert_eq!(
            false,
            Skelly::can_move(creature.current_animation_index.0 as usize)
        );

        creature.current_animation_index =
            CurrentAnimationIndex(SkellyAnimationId::LookingAround as usize);
        assert_eq!(
            false,
            Skelly::can_move(creature.current_animation_index.0 as usize)
        );
    }
}

fn setup_skelly(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let mut hm_animations = HashMapAnimationClip::new();

    for i in 0..11 {
        let id = SkellyAnimationId::from(i as usize);
        let handle = asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str());
        hm_animations.insert(id as usize, id.get_duration(), handle);
    }

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
        creature_entity_id: None,
        type_creature: TypeCreature::Skelly,
    }
}
