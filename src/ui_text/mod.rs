use bevy::prelude::*;

pub static TEXT_HELP_NO_ARM: &str = "If only I had arms to open \nthis door..";

#[derive(Component)]
pub struct TagUiText;

pub struct UiTextPlugin;
impl Plugin for UiTextPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn display_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text_value: &str,
    color: Color,
) {
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
        .insert(TagUiText);
}
