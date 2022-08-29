use crate::creatures::SceneModelState;
use crate::map::{I_SHIFT, J_SHIFT, PATH_GLTF_CHAIN, PATH_GLTF_GATE};
use crate::ui_text::{display_text, TagUiText, TEXT_HELP_NO_ARM};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

pub(crate) struct GatePlugin;
impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GateState::Closed)
            .add_startup_system(setup_gate_chain)
            .add_system(collision_with_chain_text)
            .add_system(collision_with_chain_door.after(collision_with_chain_text))
            .add_system_set(SystemSet::on_enter(GateState::Opening).with_system(tag_gate_to_remove))
            .add_system(removing_gate);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GateState {
    Closed,
    Opening,
    Opened,
}

#[derive(Component)]
pub struct TagChain;

#[derive(Component)]
pub struct TagGate;

fn setup_gate_chain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_chain = asset_server.load(PATH_GLTF_CHAIN);
    let handle_gate = asset_server.load(PATH_GLTF_GATE);

    let position_chain = 6.25 * I_SHIFT + 9.25 * J_SHIFT;

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(position_chain.x, 1.0, position_chain.z),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: handle_chain,
                transform: Transform::from_xyz(0.0, -0.6, 0.0),
                ..default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Default::default(),
                    ..default()
                })
                .insert(Collider::cone(0.05, 3.0))
                .insert(Sensor)
                .insert(TagChain);
        })
        .insert(Name::new("Chain!"));

    let gate_position = 7.0 * I_SHIFT + 9.65 * J_SHIFT;

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(gate_position.x, 0.0, gate_position.z),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: handle_gate,
                transform: Transform {
                    translation: Vec3::new(-0.1, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(0.8),
                    scale: Vec3::ONE * 0.62,
                },
                ..default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform {
                        translation: Vec3::new(0.3, 0.0, 0.3),
                        rotation: Quat::from_rotation_y(0.8),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Collider::cuboid(2.5, 2.8, 0.5))
                .insert(TagGate);
        })
        .insert(Name::new("Gate!"));
}

fn collision_with_chain_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    query_chain: Query<Entity, With<TagChain>>,
    mut query_text: Query<Entity, With<TagUiText>>,
    app_state: Res<State<SceneModelState>>,
) {
    if *app_state.current() == SceneModelState::FullBody {
        return;
    }

    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                for entity in [a, b] {
                    if let Ok(_chain) = query_chain.get(*entity) {
                        display_text(
                            commands.borrow_mut(),
                            &asset_server,
                            TEXT_HELP_NO_ARM,
                            Color::RED,
                        );
                    }
                }
            }
            CollisionEvent::Stopped(a, b, _) => {
                for entity in [a, b] {
                    if let Ok(_chain) = query_chain.get(*entity) {
                        if let Ok(text_ui) = query_text.get_single_mut() {
                            commands.entity(text_ui).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component)]
struct TagRemoveGate;

fn collision_with_chain_door(
    mut collision_events: EventReader<CollisionEvent>,
    query_chain: Query<Entity, With<TagChain>>,
    app_state: Res<State<SceneModelState>>,
    mut gate_state: ResMut<State<GateState>>,
) {
    if !(*app_state.current() == SceneModelState::FullBody
        && *gate_state.current() == GateState::Closed)
    {
        return;
    }

    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                for entity in [a, b] {
                    if let Ok(_chain) = query_chain.get(*entity) {
                        info!("Opening gate");
                        gate_state.set(GateState::Opening).unwrap();
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn tag_gate_to_remove(
    mut commands: Commands,
    query_parent: Query<&Parent>,
    query_gate: Query<Entity, With<TagGate>>,
) {
    if let Ok(child_gate) = query_gate.get_single() {
        if let Ok(gate_entity) = query_parent.get(child_gate) {
            info!("Tagging gate to remove");
            commands.entity(gate_entity.get()).insert(TagRemoveGate);
        }
    }
}

fn removing_gate(
    mut query_gate: Query<(Entity, &mut Transform), With<TagRemoveGate>>,
    mut gate_state: ResMut<State<GateState>>,
    mut commands: Commands,
) {
    if *gate_state.current() == GateState::Closed {
        return;
    }

    if let Ok((entity, mut gate_transform)) = query_gate.get_single_mut() {
        if gate_transform.translation.y <= -3.3 {
            gate_state.set(GateState::Opened).unwrap();
            commands.entity(entity).despawn_recursive();
            info!("Removing Gate!");
        } else {
            gate_transform.translation.y -= 0.01;
        }
    }
}
