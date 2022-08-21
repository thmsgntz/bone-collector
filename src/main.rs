mod animations_handler;
mod camera;
mod creatures;

use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::window::PresentMode;

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
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 5.0;

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(size / 2.0, 0.0, size / 2.0),
        ..default()
    });
}

fn main() {
    App::new()
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

        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=error,bone_collector=debug,bone_collector::animations_handler=info".into(),
            level: bevy::log::Level::DEBUG,
        })


        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations_handler::AnimationHandler)
        .add_plugin(creatures::CreaturePlugin)
        .add_startup_system(setup_light)
        .add_startup_system(setup_floor)

        .run();
}
