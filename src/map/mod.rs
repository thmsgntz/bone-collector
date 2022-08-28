use crate::creatures::SceneModelState;
use crate::ui_text::{display_text, TagUiText, TEXT_HELP_NO_ARM};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

static PATH_GLTF_FLOOR: &str = "models/floor/floor_material.gltf#Scene0";
static PATH_GLTF_CHAIN: &str = "models/hanging_wall_chains/scene.gltf#Scene0";
static PATH_GLTF_GATE: &str = "models/gate/gate.glb#Scene0";

pub(crate) const I_SHIFT: Vec3 = Vec3::new(-2.8, 0.0, 2.9);
pub(crate) const J_SHIFT: Vec3 = Vec3::new(2.9, 0.0, 2.8);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_level)
            .add_system(collison_with_chain_text)
            .add_system(collison_with_chain_door.after(collison_with_chain_text));
    }
}

#[derive(Component)]
pub struct TagChain;

#[derive(Component)]
pub struct TagGate;

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_floor = asset_server.load(PATH_GLTF_FLOOR);
    let handle_chain = asset_server.load(PATH_GLTF_CHAIN);
    let handle_gate = asset_server.load(PATH_GLTF_GATE);

    // RIGHT : Vec3::new(-2.8,0.0,2.9)
    // TOP : Vec3::new(2.9,0.0,2.8)
    // BOT : Vec3::new(-2.9,0.0,-2.8)
    // LEFT : Vec3::new(2.8,0.0,-2.9)

    generate_room_1(commands.borrow_mut(), handle_floor.clone());
    generate_room_2(commands.borrow_mut(), handle_floor.clone());
    generate_corridor_1(commands.borrow_mut(), handle_floor.clone());
    generate_corridor_2(commands.borrow_mut(), handle_floor.clone());
    generate_corridor_3(commands.borrow_mut(), handle_floor.clone());
    generate_room_3(commands.borrow_mut(), handle_floor.clone());
    generate_room_4(commands.borrow_mut(), handle_floor);

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

    let gate_position = 7.0 * I_SHIFT + 9.25 * J_SHIFT;

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(gate_position.x, 1.0, gate_position.z),
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
                .insert(Collider::cuboid(2.0, 1.0, 0.5))
                .insert(TagGate);
        })
        .insert(Name::new("Gate!"));
}

fn collison_with_chain_text(
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
                        display_text(commands.borrow_mut(), &asset_server, TEXT_HELP_NO_ARM);
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

fn collison_with_chain_door(
    // mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    query_chain: Query<Entity, With<TagChain>>,
    // app_state: Res<State<SceneModelState>>,
) {
    /* TODO: remove comment
        if *app_state.current() != SceneModelState::FullBody {
            return
        }
    */
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                for entity in [a, b] {
                    if let Ok(_chain) = query_chain.get(*entity) {
                        info!("TODO: opening door if full body");
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn generate_room_1(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 1..4 {
        for j in 0..14 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_corridor_1(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 4..6 {
        for j in 7..9 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_room_2(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 6..9 {
        for j in 3..10 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_corridor_2(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 9..11 {
        for j in 7..9 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_room_3(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 11..15 {
        for j in 3..13 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_corridor_3(command: &mut Commands, handle_floor: Handle<Scene>) {
    let i = 7.0;

    for j in 10..14 {
        generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
    }
}

fn generate_room_4(command: &mut Commands, handle_floor: Handle<Scene>) {
    for i in 6..9 {
        for j in 14..17 {
            generate_one_tile(command, handle_floor.clone(), i as f32, j as f32);
        }
    }
}

fn generate_one_tile(command: &mut Commands, handle_floor: Handle<Scene>, i: f32, j: f32) {
    let i_shift = I_SHIFT;
    let j_shift = J_SHIFT;
    let transform = (i_shift * i) + (j_shift * j);

    let size = 4.35;
    command
        .spawn_bundle(PbrBundle {
            //mesh: meshes.add(Mesh::from(shape::Plane { size })),
            transform: Transform::from_xyz(transform.x, 0.0, transform.z),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: handle_floor.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_y(0.8)),
                ..default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(PbrBundle {
                    transform: Transform {
                        translation: Default::default(),
                        rotation: Quat::from_rotation_y(0.8),
                        scale: Vec3::ONE,
                    },
                    ..default()
                })
                .insert(Collider::cuboid(size / 2.0, 0.1, size / 2.0));
        })
        .insert(RigidBody::Fixed);
}
