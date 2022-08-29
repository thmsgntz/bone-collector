use crate::map::{I_SHIFT, J_SHIFT};
use crate::ui_text::display_text;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::borrow::BorrowMut;

static TEXT_WINNING: &str = "Congratulation, you've finished the game!\n Thanks for playing :)";

pub struct EndZonePlugin;
impl Plugin for EndZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_end_zone)
            .add_system(entering_zone);
    }
}

#[derive(Component)]
struct TagEndingZone;

fn setup_end_zone(mut commands: Commands) {
    let size = 12.0;
    let position = 7.0 * I_SHIFT + 15.0 * J_SHIFT;

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(position.x, 0.5, position.z),
                rotation: Quat::from_rotation_y(0.8),
                scale: Vec3::ONE,
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size / 2.0, 0.1, size / 2.0))
        .insert(Sensor)
        .insert(TagEndingZone)
        .insert(Name::new("Ending Zone"));
}

fn entering_zone(
    mut event_collision: EventReader<CollisionEvent>,
    mut command: Commands,
    asset_server: Res<AssetServer>,
    query_zone: Query<&TagEndingZone>,
) {
    for event in event_collision.iter() {
        match event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                for entity in [entity_a, entity_b] {
                    if let Ok(_zone) = query_zone.get(*entity) {
                        info!("Entering End Zone!");
                        display_text(
                            command.borrow_mut(),
                            &asset_server,
                            TEXT_WINNING,
                            Color::YELLOW_GREEN,
                        );
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
