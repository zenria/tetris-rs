use bevy::{
    prelude::{shape::Quad, *},
    sprite::MaterialMesh2dBundle,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    player::{Action, Player},
    GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Component)]
pub struct Pause;

pub fn enter_pause(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 120.0,
        color: Color::ANTIQUE_WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("paused", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
            ..default()
        })
        .insert(Pause);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
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
        })
        .insert(Pause);
}

pub fn exit_pause(mut commands: Commands, paused_elements: Query<Entity, With<Pause>>) {
    for e in &paused_elements {
        commands.entity(e).despawn();
    }
}

pub fn pause(
    input_query: Query<&ActionState<Action>, With<Player>>,
    mut state: ResMut<State<GameState>>,
) {
    let input = input_query.single();
    if input.just_pressed(Action::Pause) {
        match state.as_ref().current() {
            GameState::GameOver => (),
            GameState::InGame => state.set(GameState::Pause).unwrap(),
            GameState::Pause => state.set(GameState::InGame).unwrap(),
        }
    }
}
