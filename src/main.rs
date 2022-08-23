mod animations_handler;
mod camera;
mod creatures;
mod directions;

use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use crate::animations_handler::{AddAnimation, HashMapAnimationClip, SceneHandle};
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
            present_mode: PresentMode::Mailbox,
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

        .run();
}
