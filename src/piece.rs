use std::ops::Add;

use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    square::{spawn_square, Square},
};

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Square,
    T,
    L,
    InvL,
    Bar,
    S,
    InvS,
}

type PiecePositions = [(i32, i32); 4];

impl Add<BoardPosition> for PiecePositions {
    type Output = Self;

    fn add(self, rhs: BoardPosition) -> Self::Output {
        [self[0] + rhs, self[1] + rhs, self[2] + rhs, self[3] + rhs]
    }
}

impl PieceType {
    fn color(&self) -> Color {
        match self {
            PieceType::Square => Color::RED,
            PieceType::T => Color::YELLOW,
            PieceType::L => Color::BLUE,
            PieceType::InvL => Color::GREEN,
            PieceType::Bar => Color::CYAN,
            PieceType::S => Color::FUCHSIA,
            PieceType::InvS => Color::VIOLET,
        }
    }

    pub fn anchor(&self) -> BoardPosition {
        match self {
            PieceType::Square => BoardPosition::new(0, 0),
            PieceType::T => BoardPosition::new(-1, 0),
            PieceType::L => BoardPosition::new(-1, 0),
            PieceType::InvL => BoardPosition::new(-1, 0),
            PieceType::Bar => BoardPosition::new(0, 0),
            PieceType::S => BoardPosition::new(0, 0),
            PieceType::InvS => BoardPosition::new(0, 0),
        }
    }

    pub fn square_pos(&self, orientation: Orientation) -> PiecePositions {
        match self {
            PieceType::Square => [(0, 0), (1, 0), (0, -1), (1, -1)],
            PieceType::T => match orientation {
                Orientation::Up => [(1, 0), (0, -1), (1, -1), (2, -1)],
                Orientation::Left => [(0, -1), (1, 0), (1, -1), (1, -2)],
                Orientation::Bottom => [(0, -1), (1, -1), (2, -1), (1, -2)],
                Orientation::Right => [(1, 0), (1, -1), (1, -2), (2, -1)],
            },
            PieceType::L => match orientation {
                Orientation::Up => [(0, 0), (1, 0), (2, 0), (2, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, -2), (1, 0)],
                Orientation::Bottom => [(0, 0), (0, -1), (1, -1), (2, -1)],
                Orientation::Right => [(1, 0), (1, -1), (1, -2), (0, -2)],
            },

            PieceType::InvL => match orientation {
                Orientation::Up => [(0, 0), (1, 0), (2, 0), (0, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, -2), (1, -2)],
                Orientation::Bottom => [(2, 0), (0, -1), (1, -1), (2, -1)],
                Orientation::Right => [(0, 0), (1, 0), (1, -1), (1, -2)],
            },
            PieceType::Bar => match orientation {
                // horizontal
                Orientation::Up | Orientation::Bottom => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                // vertical
                Orientation::Left | Orientation::Right => [(1, 0), (1, 1), (1, 2), (1, 3)],
            },
            PieceType::S => match orientation {
                Orientation::Up | Orientation::Bottom => [(1, 0), (2, 0), (0, -1), (1, -1)],
                Orientation::Left | Orientation::Right => [(0, 0), (0, -1), (1, -1), (1, -2)],
            },
            PieceType::InvS => match orientation {
                Orientation::Up | Orientation::Bottom => [(0, 0), (1, 0), (1, -1), (2, -1)],
                Orientation::Left | Orientation::Right => [(0, -1), (0, -2), (1, 0), (1, -1)],
            },
        }
    }
}

// this made selecting a random piece easy using rand::random()
impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..7) {
            // rand 0.8
            0 => PieceType::Square,
            1 => PieceType::T,
            2 => PieceType::L,
            3 => PieceType::InvL,
            4 => PieceType::S,
            5 => PieceType::InvS,
            _ => PieceType::Bar,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Up,
    Left,
    Bottom,
    Right,
}

impl Orientation {
    pub fn apply_rotation(&self, rotation: Rotation) -> Orientation {
        match self {
            Orientation::Up => match rotation {
                Rotation::Clock => Orientation::Right,
                Rotation::Anti => Orientation::Left,
            },
            Orientation::Left => match rotation {
                Rotation::Clock => Orientation::Up,
                Rotation::Anti => Orientation::Bottom,
            },
            Orientation::Bottom => match rotation {
                Rotation::Clock => Orientation::Left,
                Rotation::Anti => Orientation::Right,
            },
            Orientation::Right => match rotation {
                Rotation::Clock => Orientation::Bottom,
                Rotation::Anti => Orientation::Up,
            },
        }
    }
}

/// Marker components for squares that belong to the current moving piece
#[derive(Component, Clone, Copy)]
pub struct PieceSquare;

/// The actual moving piece that goes down and can be moved/rotated
#[derive(Component, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub orientation: Orientation,
    /// helper position to ease rotation computation
    pub position: BoardPosition,
}

pub enum Rotation {
    Clock,
    Anti,
}

impl Piece {
    fn square_pos(&self) -> PiecePositions {
        self.piece_type.square_pos(self.orientation) + self.piece_type.anchor()
    }
}

pub fn spawn_random_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let piece = Piece {
        piece_type: PieceType::Bar, //rand::random(),
        orientation: Orientation::Up,
        position: BoardPosition::new(BOARD_WIDTH / 2, BOARD_HEIGHT),
    };
    // spawn the squares
    for square_pos in piece.square_pos() {
        spawn_square(
            commands,
            meshes,
            materials,
            piece.position + square_pos,
            piece.piece_type.color(),
            Square,
            Some(PieceSquare),
        );
    }
    // spawn the actual piece
    commands.spawn().insert(piece);
}
