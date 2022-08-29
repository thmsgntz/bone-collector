use crate::creatures::SceneModelState;
use bevy::prelude::*;
use std::time::Duration;

pub static TEXT_HELP_NO_ARM: &str = "If only I had arms to open \nthis door..";
pub static TEXT_NOW_RUN: &str = "Now I can run..";

#[derive(Component)]
pub struct TagTextNowRun(Timer);

#[derive(Component)]
pub struct TagUiText;

pub struct UiTextPlugin;
impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneModelState::HalfBody).with_system(display_text_now_run),
        )
        .add_system(update_or_remove_text_now_run);
    }
}

fn update_or_remove_text_now_run(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TagTextNowRun)>,
) {
    if let Ok((entity, mut timer_ui)) = query.get_single_mut() {
        timer_ui.0.tick(time.delta());

        if timer_ui.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn display_text_now_run(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn_bundle(NodeBundle {
            node: Default::default(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::SpaceEvenly,
                position: UiRect::new(
                    Val::Percent(25.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(20.0),
                ),
                ..default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    TEXT_NOW_RUN,
                    TextStyle {
                        font: handle_font,
                        font_size: 25.0,
                        color: Color::RED,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..default()
                })
                .with_text_alignment(TextAlignment::CENTER),
            );
        })
        .insert(TagTextNowRun(Timer::new(
            Duration::from_secs_f32(5.0),
            false,
        )));
}

pub fn display_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text_value: &str,
    color: Color,
) {
    let handle_font = asset_server.load("fonts/FiraMono-Medium.ttf");

    // horrible but no time :)
    let shift_left = if color == Color::RED { 25.0 } else { 18.0 };

    commands
        .spawn_bundle(NodeBundle {
            node: Default::default(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::SpaceEvenly,
                position: UiRect::new(
                    Val::Percent(shift_left),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(20.0),
                ),
                ..default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    text_value,
                    TextStyle {
                        font: handle_font,
                        font_size: 25.0,
                        color,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..default()
                })
                .with_text_alignment(TextAlignment::CENTER),
            );
        })
        .insert(TagUiText)
        .insert(Name::new("Text I jk"));
}
