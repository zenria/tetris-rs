use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    square::{spawn_square, Square},
};

pub enum PieceType {
    Square,
    T,
    L,
    InvL,
    Bar,
}

impl PieceType {
    fn color(&self) -> Color {
        match self {
            PieceType::Square => Color::RED,
            PieceType::T => Color::YELLOW,
            PieceType::L => Color::BLUE,
            PieceType::InvL => Color::GREEN,
            PieceType::Bar => Color::CYAN,
        }
    }
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..5) {
            // rand 0.8
            0 => PieceType::Square,
            1 => PieceType::T,
            2 => PieceType::L,
            3 => PieceType::InvL,
            _ => PieceType::Bar,
        }
    }
}

enum Orientation {
    Up,
    Left,
    Bottom,
    Right,
}

#[derive(Component)]
pub struct Piece {
    pub piece_type: PieceType,
}

pub fn spawn_random_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let piece = Piece {
        piece_type: rand::random(),
    };
    spawn_square(
        commands,
        meshes,
        materials,
        BoardPosition::new(BOARD_WIDTH / 2, BOARD_HEIGHT),
        piece.piece_type.color(),
        Square,
        Some(piece),
    );
}
