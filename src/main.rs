#![feature(duration_consts_float)]
use std::time::Duration;

use bevy::prelude::*;
use board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use piece::{spawn_random_piece, Piece};
use player::{spawn_player, Action, Level, Player};
use square::{spawn_square, Square, SQ_TOTAL_SIZE};

const KEY_REPEAT_DELAY: Duration = Duration::from_secs_f32(0.1);
const FAST_DOWN_DELAY: Duration = Duration::from_secs_f64(0.03);

fn main() {
    let level = Level::default();

    App::new()
        .add_plugins(DefaultPlugins)
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_event::<PieceHasStoppedEvent>()
        .add_system(bevy::window::close_on_esc)
        .add_system(move_down)
        .add_system(spawn_new_on_stopped)
        .add_system(move_down_faster)
        .add_system(stop_fast_move_down_on_collision)
        .add_system(move_horizontally)
        .insert_resource(MoveDownTimer {
            timer: Timer::from_seconds(level.get_down_duration().as_secs_f32(), true),
        })
        .insert_resource(MoveHoritontallyTimer {
            timer: Timer::new(KEY_REPEAT_DELAY, true),
        })
        .insert_resource(level)
        .run();
}

mod board;
mod piece;
mod player;
mod square;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    spawn_player(&mut commands);

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
struct MoveHoritontallyTimer {
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
    for (_, fixed_bp) in &fixed_query {
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

fn spawn_new_on_stopped(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<PieceHasStoppedEvent>,
) {
    for _ev in event_reader.iter() {
        spawn_random_piece(&mut commands, &mut meshes, &mut materials)
    }
}

fn move_horizontally(
    query: Query<&ActionState<Action>, With<Player>>,
    fixed_query: Query<(&Square, &mut BoardPosition), Without<Piece>>,
    mut moving_query: Query<(Entity, &Square, &mut BoardPosition, &mut Transform), With<Piece>>,
    mut timer: ResMut<MoveHoritontallyTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());

    let action_state = query.single();

    // Just pressed the key
    let mut direction: Option<i32> = if action_state.just_pressed(Action::Left) {
        Some(-1)
    } else if action_state.just_pressed(Action::Right) {
        Some(1)
    } else {
        None
    };

    match direction {
        Some(_) => timer.timer.reset(),
        None => {
            if timer.timer.just_finished() {
                // recompute direction if keys are still pressed
                if action_state.pressed(Action::Left) {
                    direction = Some(-1);
                } else if action_state.pressed(Action::Right) {
                    direction = Some(1);
                }
            }
        }
    }

    if let Some(direction) = direction {
        let mut collided = false;
        'outer: for (_, fixed_bp) in &fixed_query {
            for (_, _, moving_bp, _) in &moving_query {
                if fixed_bp.x == moving_bp.x + direction && fixed_bp.y == moving_bp.y {
                    // collision: the piece must not go left or right...
                    collided = true;
                    break 'outer;
                }
            }
        }
        if !collided {
            for (_, _, mut bp, mut tr) in moving_query.iter_mut() {
                bp.x += direction;
                tr.translation.x += SQ_TOTAL_SIZE * direction as f32;
            }
        }
    }
}

fn move_down_faster(
    level: Res<Level>,
    mut timer: ResMut<MoveDownTimer>,
    query: Query<&ActionState<Action>, With<Player>>,
) {
    let action_state = query.single();

    if action_state.just_pressed(Action::Down) {
        timer.timer.set_duration(FAST_DOWN_DELAY)
    }
    if action_state.just_released(Action::Down) {
        timer.timer.set_duration(level.get_down_duration())
    }
}

fn stop_fast_move_down_on_collision(
    level: Res<Level>,
    mut timer: ResMut<MoveDownTimer>,
    mut event_reader: EventReader<PieceHasStoppedEvent>,
) {
    for _ev in event_reader.iter() {
        timer.timer.set_duration(level.get_down_duration())
    }
}
