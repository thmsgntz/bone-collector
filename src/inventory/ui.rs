use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_image = asset_server.load("icon.png");
    let handle_font =  asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            node: Default::default(),
            style: Style {
                size: Size::new(Val::Percent(75.0), Val::Percent(15.0)),
                justify_content: JustifyContent::SpaceEvenly,
                position: UiRect::new(
                    Val::Percent(12.5),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                    Val::Percent(1.0),
                ),
                ..default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.5)), // Todo : alpha = 255 quand terminé
            ..default()
        })

        // 1st box : Bones
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                image: UiImage(handle_image.clone()),

                ..default()
            })
            .with_children(|parent| {
                // text
                parent.spawn_bundle(
                    TextBundle::from_section(
                        "Bones : 0",
                        TextStyle {
                            font: handle_font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    )
                        .with_style(Style {
                            align_self: AlignSelf::FlexStart,
                            position: UiRect::new (
                                Val::Percent(50.0),
                                Val::Percent(0.0),
                                Val::Percent(0.0),
                                Val::Percent(0.0),
                            ),
                            ..default()
                        }),
                );
            });
        });
}
