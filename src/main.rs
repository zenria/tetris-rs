use bevy::prelude::*;
use board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH};
use square::{spawn_square, Square};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

mod board;
mod square;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    for i in 0..=BOARD_WIDTH {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(i, 0),
            Color::BLACK,
            Square {
                state: square::SquareState::Fixed,
            },
        );
    }
    for i in 1..=BOARD_HEIGHT {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(0, i),
            Color::BLACK,
            Square {
                state: square::SquareState::Fixed,
            },
        );
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH, i),
            Color::BLACK,
            Square {
                state: square::SquareState::Fixed,
            },
        );
    }

    spawn_square(
        &mut commands,
        &mut meshes,
        &mut materials,
        BoardPosition::new(2, 1),
        Color::GREEN,
        Square {
            state: square::SquareState::Normal,
        },
    );

    spawn_square(
        &mut commands,
        &mut meshes,
        &mut materials,
        BoardPosition::new(1, 1),
        Color::GREEN,
        Square {
            state: square::SquareState::Normal,
        },
    );
    spawn_square(
        &mut commands,
        &mut meshes,
        &mut materials,
        BoardPosition::new(1, 2),
        Color::BLUE,
        Square {
            state: square::SquareState::Normal,
        },
    );
    spawn_square(
        &mut commands,
        &mut meshes,
        &mut materials,
        BoardPosition::new(1, 3),
        Color::RED,
        Square {
            state: square::SquareState::Normal,
        },
    );
}
