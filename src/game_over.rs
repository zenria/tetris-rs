use bevy::{
    prelude::{shape::Quad, *},
    sprite::MaterialMesh2dBundle,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
pub fn game_over(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 120.0,
        color: Color::RED,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("GAME\nOVER", text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..default()
    });

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(
                Quad {
                    size: Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..Default::default()
                }
                .into(),
            )
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgba(0., 0., 0., 0.5))),
        transform: Transform::from_translation(Vec3::new(0., 0., 5.)),
        ..default()
    });
}
