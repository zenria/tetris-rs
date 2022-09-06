use bevy::prelude::*;
use board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH};
use piece::{spawn_random_piece, Piece, PieceType};
use square::{spawn_square, Square, SQ_TOTAL_SIZE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_event::<PieceHasStoppedEvent>()
        .add_system(bevy::window::close_on_esc)
        .add_system(move_down)
        .add_system(debug_has_stopped)
        .add_system(spawn_new_on_stopped)
        .insert_resource(MoveDownTimer {
            timer: Timer::from_seconds(0.1, true),
        })
        .run();
}

mod board;
mod piece;
mod square;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // setup board

    for i in 0..=(BOARD_WIDTH + 1) {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(i, 0),
            Color::BLACK,
            Square,
            None,
        );
    }
    for i in 1..=BOARD_HEIGHT {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(0, i),
            Color::BLACK,
            Square,
            None,
        );
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 1, i),
            Color::BLACK,
            Square,
            None,
        );
    }

    // spawn the first piece
    spawn_random_piece(&mut commands, &mut meshes, &mut materials);
}

struct MoveDownTimer {
    timer: Timer,
}

fn move_down(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveDownTimer>,
    mut piece_has_stopped_event_writer: EventWriter<PieceHasStoppedEvent>,
    fixed_query: Query<(&Square, &mut BoardPosition), Without<Piece>>,
    mut moving_query: Query<(Entity, &Square, &mut BoardPosition, &mut Transform), With<Piece>>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.finished() {
        return;
    }
    // check for collisions
    let mut collided = false;
    let mut game_over = false;
    'outer: for (_, fixed_bp) in &fixed_query {
        for (_, _, moving_bp, _) in &moving_query {
            if fixed_bp.x == moving_bp.x && fixed_bp.y == moving_bp.y {
                // Collision even before moving the piece: game over!
                game_over = true;
            }
            if fixed_bp.x == moving_bp.x && fixed_bp.y == moving_bp.y - 1 {
                // collision: the piece must not go down...
                collided = true;
            }
        }
    }
    if collided || game_over {
        // TODO
        // count score
        for (entity, _, _, _) in moving_query.iter_mut() {
            // transform the Piece into fixed squares
            commands.entity(entity).remove::<Piece>();
        }
        if !game_over {
            // no need to spawn something new on game over
            piece_has_stopped_event_writer.send_default();
        }
    } else {
        for (_, _, mut bp, mut tr) in moving_query.iter_mut() {
            bp.y -= 1;
            tr.translation.y -= SQ_TOTAL_SIZE;
        }
    }
}

/// When a piece has stopped by hitting something concrete
#[derive(Default)]
struct PieceHasStoppedEvent;

fn debug_has_stopped(mut event_reader: EventReader<PieceHasStoppedEvent>) {
    for ev in event_reader.iter() {
        eprintln!("Piece has hit something hard!");
    }
}

fn spawn_new_on_stopped(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<PieceHasStoppedEvent>,
) {
    for ev in event_reader.iter() {
        spawn_random_piece(&mut commands, &mut meshes, &mut materials)
    }
}
