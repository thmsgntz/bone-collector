mod animations_handler;
mod camera;
mod creatures;
mod directions;
mod inventory;
mod map;
mod ui_text;

use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::animations_handler::{AddAnimation, HashMapAnimationClip, SceneHandle};
use crate::creatures::skelly::SkellyAnimationId;
use bevy_rapier3d::prelude::*;

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
    const HALF_SIZE: f32 = 10.0;
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("SunLight"));
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
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })

        /* Diagnostics */
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())

        /* Login*/
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=error,bone_collector=debug,bone_collector::animations_handler=info,bevy_animation=warn".into(),
            level: bevy::log::Level::DEBUG,
        })

        /* DefaultPlugins */
        .add_plugins(DefaultPlugins)

        /* EGUI info */
        //.add_plugin(WorldInspectorPlugin::new())

        /* Rapier */
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())

        /* My stuff */
        .add_plugin(map::MapPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations_handler::AnimationHandler)
        .add_plugin(creatures::CreaturePlugin)
        .add_plugin(inventory::InventoryPlugin)
        .add_plugin(ui_text::UiTextPlugin)
        .add_startup_system(setup_light)

        .run();
}
