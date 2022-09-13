#![feature(duration_consts_float)]
use std::time::Duration;

use bevy::{prelude::*, window::PresentMode};
use board::{Board, BoardPosition, BOARD_HEIGHT, BOARD_WIDTH};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use piece::{spawn_next_piece, Piece, PieceSquare, Rotation};
use player::{spawn_player, Action, Level, Player};
use square::{spawn_square, Square, Wall, SQ_TOTAL_SIZE};

const FIRST_REPEAT_DELAY: Duration = Duration::from_secs_f32(0.25);
const KEY_REPEAT_DELAY: Duration = Duration::from_secs_f32(0.1);
const FAST_DOWN_DELAY: Duration = Duration::from_secs_f64(0.03);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    GameOver,
    InGame,
    Pause,
}

#[derive(Default)]
pub struct SpawnPieceEvent;

const WINDOW_WIDTH: f32 = (BOARD_WIDTH + 12) as f32 * SQ_TOTAL_SIZE;
const WINDOW_HEIGHT: f32 = (BOARD_HEIGHT + 2) as f32 * SQ_TOTAL_SIZE;

fn main() {
    let level = Level::default();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Oxidized Tetris".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_state(GameState::InGame)
        .add_startup_system(setup)
        .add_event::<PieceHasStoppedEvent>()
        .add_event::<SpawnPieceEvent>()
        .add_system(bevy::window::close_on_esc)
        .add_system(pause::pause)
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over::game_over))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(move_down)
                .with_system(move_horizontally)
                // detect complete line must be executed before move_down
                // because move down can remove a component that is used within a condition
                // for line detection. if the detection happens after a collision has been
                // detected and the PieceSquare has been removed, the line detection system will
                // not see the removal of the component until next tick
                .with_system(detect_complete_lines.before(move_down))
                .with_system(spawn_new_on_stopped)
                .with_system(spawn_next_piece)
                .with_system(move_down_faster)
                .with_system(rotate)
                .with_system(stop_fast_move_down_on_collision),
        )
        .add_system_set(SystemSet::on_enter(GameState::Pause).with_system(pause::enter_pause))
        .add_system_set(SystemSet::on_exit(GameState::Pause).with_system(pause::exit_pause))
        .insert_resource(MoveDownTimer {
            timer: Timer::from_seconds(level.get_down_duration().as_secs_f32(), true),
        })
        .insert_resource(MoveHorizontallyTimer {
            timer: Timer::new(FIRST_REPEAT_DELAY, true),
        })
        .insert_resource(level)
        .run();
}

mod board;
mod game_over;
mod pause;
mod piece;
mod player;
mod square;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spwan_piece_event_writer: EventWriter<SpawnPieceEvent>,
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
            Some(Wall),
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
            Some(Wall),
        );
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 1, i),
            Color::BLACK,
            Square,
            Some(Wall),
        );
    }

    // setup next piece walls

    for i in 0..6 {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 3 + i, 0),
            Color::BLACK,
            Square,
            Some(Wall),
        );
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 3 + i, 5),
            Color::BLACK,
            Square,
            Some(Wall),
        );
    }
    for i in 0..4 {
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 3, 1 + i),
            Color::BLACK,
            Square,
            Some(Wall),
        );
        spawn_square(
            &mut commands,
            &mut meshes,
            &mut materials,
            BoardPosition::new(BOARD_WIDTH + 3 + 5, 1 + i),
            Color::BLACK,
            Square,
            Some(Wall),
        );
    }

    // spawn the first piece
    spwan_piece_event_writer.send_default();
}

/// The timer used to mode down pieces
struct MoveDownTimer {
    timer: Timer,
}
/// The timer to move right/left piece while the left or
/// right key is pressed
struct MoveHorizontallyTimer {
    timer: Timer,
}

fn move_down(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveDownTimer>,
    mut piece_has_stopped_event_writer: EventWriter<PieceHasStoppedEvent>,
    fixed_query: Query<(&Square, &mut BoardPosition), Without<PieceSquare>>,
    mut moving_query: Query<
        (Entity, &Square, &mut BoardPosition, &mut Transform),
        With<PieceSquare>,
    >,
    mut piece: Query<(Entity, &mut Piece)>,
    mut state: ResMut<State<GameState>>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
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
            commands.entity(entity).remove::<PieceSquare>();
        }
        // despawn the current piece
        commands.entity(piece.single().0).despawn();

        if !game_over {
            // no need to spawn something new on game over
            piece_has_stopped_event_writer.send_default();
        } else {
            // game over
            let _ = state.set(GameState::GameOver);
        }
    } else {
        for (_, _, mut bp, mut tr) in moving_query.iter_mut() {
            bp.y -= 1;
            tr.translation.y -= SQ_TOTAL_SIZE;
        }
        for (_, mut piece) in &mut piece {
            piece.position.y -= 1;
        }
    }
}

/// When a piece has stopped by hitting something concrete
#[derive(Default)]
struct PieceHasStoppedEvent;

fn spawn_new_on_stopped(
    mut event_reader: EventReader<PieceHasStoppedEvent>,
    mut spawn_piece_writer: EventWriter<SpawnPieceEvent>,
) {
    for _ev in event_reader.iter() {
        spawn_piece_writer.send_default();
    }
}

fn rotate(
    input_query: Query<&ActionState<Action>, With<Player>>,
    mut moving_query: Query<(&Square, &mut BoardPosition, &mut Transform), With<PieceSquare>>,
    fixed_query: Query<(&Square, &BoardPosition), Without<PieceSquare>>,
    mut piece: Query<&mut Piece>,
) {
    if let Ok(mut piece) = piece.get_single_mut() {
        let action = input_query.single();
        let rotation = if action.just_pressed(Action::RotateAnti) {
            Some(Rotation::Anti)
        } else if action.just_pressed(Action::RotateClock) {
            Some(Rotation::Clock)
        } else {
            None
        };
        if let Some(rotation) = rotation {
            // gogogo let's go
            let new_orientation = piece.orientation.apply_rotation(rotation);
            // check if the new orientation collides with something concrete
            let new_positions = piece.piece_type.square_pos(new_orientation)
                + piece.piece_type.anchor()
                + piece.position;

            let board = fixed_query.iter().map(|(_, bp)| *bp).collect::<Board>();
            for pos in new_positions {
                if board.is_concrete(pos) {
                    // cannot rotate
                    return;
                }
            }
            // nothing is in the rotation way! let's change the position of all squares
            let mut moving_square = moving_query.iter_mut();
            for pos in new_positions {
                // by construction there are exactly 4 moving square, so we can safely
                // unwrap at each iteration
                let (_, mut bp, mut tr) = moving_square.next().unwrap();
                *bp = pos.into();
                *tr = bp.as_ref().to_real_position();
            }
            piece.orientation = new_orientation;

            println!("New orientaation: {:?}", piece);
        }
    }
}

fn move_horizontally(
    query: Query<&ActionState<Action>, With<Player>>,
    fixed_query: Query<(&Square, &mut BoardPosition), Without<PieceSquare>>,
    mut moving_query: Query<
        (Entity, &Square, &mut BoardPosition, &mut Transform),
        With<PieceSquare>,
    >,
    mut timer: ResMut<MoveHorizontallyTimer>,
    time: Res<Time>,
    mut piece: Query<&mut Piece>,
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
        Some(_) => {
            // left or right ; just pressed
            timer.timer.set_duration(FIRST_REPEAT_DELAY);
            timer.timer.reset();
        }
        None => {
            // is the key still pressed?
            if timer.timer.just_finished() {
                if timer.timer.duration() == FIRST_REPEAT_DELAY {
                    // adjust timer duration after first delay
                    timer.timer.set_duration(KEY_REPEAT_DELAY);
                    timer.timer.reset();
                }
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
            // there is 0 (gane over) or 1 (running) piece on the board...
            for mut piece in &mut piece {
                piece.position.x += direction;
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

fn detect_complete_lines(
    mut commands: Commands,
    mut event_reader: EventReader<PieceHasStoppedEvent>,
    mut fixed_query: Query<
        (Entity, &Square, &mut BoardPosition, &mut Transform),
        (Without<PieceSquare>, Without<Wall>),
    >,
) {
    for _ev in event_reader.iter() {
        let board = fixed_query
            .iter()
            .map(|(_, _, bp, _)| *bp)
            .collect::<Board>();

        let full_lines = (1..=BOARD_HEIGHT)
            .into_iter()
            .filter(|line| board.is_line_full(*line))
            .collect::<Vec<_>>();

        //println!("BOARD:\n{}", board);
        println!("Full lines: {:?}", full_lines);

        for (entity, _, mut bp, mut tr) in &mut fixed_query {
            if full_lines.contains(&bp.y) {
                //remove squares from completed lines!
                commands.entity(entity).despawn_recursive();
            } else {
                // how many lines have been completed below the current
                // position?
                let completed_below = full_lines.iter().filter(|line| **line < bp.y).count() as i32;
                // move down!!!
                bp.y -= completed_below;
                tr.translation.y -= completed_below as f32 * SQ_TOTAL_SIZE;
            }
        }
    }
}
