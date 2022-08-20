use crate::animations_handler::{AddAnimation, ChangeAnimation, HashMapAnimationClip, SceneHandle};
use crate::creatures::{Creature, CreatureTrait, CurrentAnimationIndex, Player, TypeCreature};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum SkellyAnimationId {
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

impl Into<usize> for SkellyAnimationId {
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
}

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

impl Into<CurrentAnimationIndex> for SkellyAnimationId {
    fn into(self) -> CurrentAnimationIndex {
        CurrentAnimationIndex(self.into())
    }
}

impl SkellyAnimationId {
    fn get_duration(&self) -> f32 {
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
        let mut skelly_scene_handle = setup_skelly(&asset_server, "models/skeleton/scene.gltf");

        // Skeleton
        let skelly_id = commands
            .spawn()
            .insert_bundle(PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::ONE * 0.6,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SceneBundle {
                    scene: skelly_scene_handle.handle.clone(),
                    ..default()
                });
            })
            .insert(Creature {
                type_creature: TypeCreature::Skelly,
                current_animation_index: SkellyAnimationId::Idle.into(),
            })
            .insert(Player)
            .id();

        skelly_scene_handle.creature_entity_id = Some(skelly_id.id());

        event_writer.send(AddAnimation {
            scene_handler: skelly_scene_handle,
        });
    }

    fn update_animation(
        target: u32,
        index_animation: usize,
        event_writer: &mut EventWriter<ChangeAnimation>,
    ) {
        info!(
            "calling with {:#?} {}",
            SkellyAnimationId::from(index_animation),
            index_animation
        );
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
            index: new_animation.into(),
            repeat,
        });
    }
}

fn setup_skelly(asset_server: &Res<AssetServer>, scene_path: &str) -> SceneHandle {
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let mut hm_animations = HashMapAnimationClip::new();

    for i in 0..11 {
        let id = SkellyAnimationId::from(i as usize);
        let handle = asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str());
        hm_animations.insert(id.into(), id.get_duration(), handle);
    }

    SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
        creature_entity_id: None,
    }
}
