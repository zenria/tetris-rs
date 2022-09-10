use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    square::{spawn_square, Square},
};

#[derive(Clone, Copy)]
pub enum PieceType {
    Square,
    T,
    L,
    InvL,
    Bar,
    S,
    InvS,
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

    fn initial_square_pos(&self) -> [(i32, i32); 4] {
        match self {
            PieceType::Square => [(0, 0), (1, 0), (0, -1), (1, -1)],
            PieceType::T => [(0, 0), (-1, -1), (0, -1), (1, -1)],
            PieceType::L => [(-1, 0), (0, 0), (1, 0), (1, -1)],
            PieceType::InvL => [(-1, 0), (0, 0), (1, 0), (-1, -1)],
            PieceType::Bar => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            PieceType::S => [(-1, 0), (0, 0), (0, -1), (1, -1)],
            PieceType::InvS => [(0, 0), (1, 0), (-1, -1), (0, -1)],
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

#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Left,
    Bottom,
    Right,
}

/// The actuel piece that goes down and can be moved/rotated
#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub orientation: Orientation,
}

pub fn spawn_random_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let piece = Piece {
        piece_type: rand::random(),
        orientation: Orientation::Up,
    };
    for (bp_x, bp_y) in piece.piece_type.initial_square_pos() {
        spawn_square(
            commands,
            meshes,
            materials,
            BoardPosition::new(BOARD_WIDTH / 2 + bp_x, BOARD_HEIGHT + bp_y),
            piece.piece_type.color(),
            Square,
            Some(piece),
        );
    }
}
