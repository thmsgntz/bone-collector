use bevy::{prelude::*, render::camera::ScalingMode};

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 4.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-3.0, 5.0, -3.0)
            .looking_at(Vec3::new(4.0, 0.0, 4.0), Vec3::Y),
        ..default()
    });
}
