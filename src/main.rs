mod animations_handler;
mod camera;
mod creatures;
mod directions;

use bevy::log::LogSettings;
use bevy::prelude::*;

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use crate::animations_handler::{AddAnimation, HashMapAnimationClip, SceneHandle};
use crate::creatures::skelly;
use crate::creatures::skelly::SkellyAnimationId;

mod settings {
    use bevy::window::WindowMode;

    pub static NAME: &str = "BoneCollector!";
    pub const WINDOW_WIDTH: f32 = 800.;
    pub const WINDOW_HEIGHT: f32 = 600.;
    pub const WINDOW_POSITION_X: f32 = 50.;
    pub const WINDOW_POSITION_Y: f32 = 25.;
    pub const WINDOW_MODE: WindowMode = WindowMode::Windowed;
}

fn setup_light(mut commands: Commands) {
    // light
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 5.0),
            ..default()
        })
        .insert(Name::new("PointLight"));
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 10.0;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size / 2.0, 0.1, size / 2.0))
        .insert(Name::new("Floor"));
}

fn spawn_bone(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<AddAnimation>,
) {
    let scene_path = skelly::GLTF_PATH_BONE;
    let asset_scene_handle = asset_server.load(format!("{}{}", scene_path, "#Scene0").as_str());

    let mut hm_animations = HashMapAnimationClip::new();

    for i in 0..1 {
        let id = i;
        let handle = asset_server.load(format!("{}#Animation{}", scene_path, id as usize).as_str());
        hm_animations.insert(id as usize, SkellyAnimationId::Idle.get_duration(), handle);
    }

    let mut scene = SceneHandle {
        handle: asset_scene_handle,
        vec_animations: hm_animations,
        creature_entity_id: None,
    };

    let bone_id = commands.spawn()
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(2.0, 0.0, 2.0),
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
                    scale: Vec3::ONE * 1.0,
                },
                ..default()
            });
        }).id();

    scene.creature_entity_id = Some(bone_id.id());

    event_writer.send(AddAnimation {
        scene_handler: scene,
    });
}

fn main() {
    App::new()

        /* Window */
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: settings::NAME.parse().unwrap(),
            width: settings::WINDOW_WIDTH,
            height: settings::WINDOW_HEIGHT,
            position: WindowPosition::At(Vec2::new(settings::WINDOW_POSITION_X, settings::WINDOW_POSITION_Y)),
            mode: settings::WINDOW_MODE,
            //present_mode: PresentMode::Mailbox,
            ..Default::default()
        })

        /* Diagnostics */
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())

        /* Login*/
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=error,bone_collector=debug,bone_collector::animations_handler=info".into(),
            level: bevy::log::Level::DEBUG,
        })

        /* DefaultPlugins */
        .add_plugins(DefaultPlugins)

        /* EGUI info */
        .add_plugin(WorldInspectorPlugin::new())
        //.register_type::<Creature>()

        /* Rapier */
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())

        /* My stuff */
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations_handler::AnimationHandler)
        .add_plugin(creatures::CreaturePlugin)
        .add_startup_system(setup_light)
        .add_startup_system(setup_floor)
        .add_startup_system(spawn_bone)

        .run();
}
