use crate::creatures::Player;
use bevy::{prelude::*, render::camera::ScalingMode};

/// camera distance from the player
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub(crate) struct ShiftFromPlayer(f32);

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource::<ShiftFromPlayer>(ShiftFromPlayer(5.0))
            //    .add_startup_system(draw_repere)
            .add_system(camera_following_player);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 4.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(-5.0, 5.0, -5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Name::new("Camera 3D"));
}

fn camera_following_player(
    shift_value: Res<ShiftFromPlayer>,
    mut query_camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut query_player: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    if let Ok(player_transform) = query_player.get_single_mut() {
        let mut camera_transform = query_camera
            .get_single_mut()
            .expect("Error while querying camera");
        let player_translation = player_transform.translation;

        let shift = shift_value.0;

        *camera_transform = Transform::from_xyz(
            player_translation.x - shift,
            camera_transform.translation.y,
            player_translation.z - shift,
        )
        .looking_at(player_translation, Vec3::Y);
    }
}

/*
fn draw_repere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 1,
            })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Repère 0.0.0"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 1,
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Repère 1.0.0"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 1,
            })),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Name::new("Repère 0.1.0"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 1,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(Name::new("Repère 0.0.1"));
}
*/
