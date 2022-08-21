use crate::animations_handler::{AddAnimation, ChangeAnimation};
use crate::creatures::skelly::Skelly;
use bevy::math::vec3;

use crate::directions;
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

pub(crate) mod skelly;

// const ENTITY_SPEED: f32 = 2.0;
// const ENTITY_SPEED_ROTATION: f32 = 0.1;

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

/// Player marker
#[derive(Component)]
pub(crate) struct Player;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_skelly)
            .add_system(keyboard_control);
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

#[derive(Component, Copy, Clone)]
/// Contient l'index de l'animation en cours
/// Mis à jour par animations_handler:update_animation
pub struct CurrentAnimationIndex(pub usize);

impl From<usize> for CurrentAnimationIndex {
    fn from(a: usize) -> Self {
        Self(a)
    }
}

pub enum TypeCreature {
    Skelly,
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
            TypeCreature::Skelly => {
                Skelly::update_animation(target, index_animation, event_writer);
            }
        }
    }
}

fn keyboard_control(
    mut event_writer: EventWriter<ChangeAnimation>,
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
        // Returns if vector_direction is 0
        if vector_direction == Vec3::ZERO {
            // TODO: if the player.currentanimation was walking, then idle
            player_velocity.linvel = vec3(0.0, player_velocity.linvel.y, 0.0);
            return;
        }

        // Returns if the player can not move
        match player_creature.type_creature {
            TypeCreature::Skelly => {
                if !Skelly::can_move(player_creature.current_animation_index.0) {
                    return;
                }
            }
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

        // TODO: better comparison please
        if player_creature.current_animation_index.0 != skelly::SkellyAnimationId::Walk as usize {
            event_writer.send(ChangeAnimation {
                target: entity.id(),
                index: skelly::SkellyAnimationId::Walk as usize,
                repeat: true,
            });
        }
    }
}
