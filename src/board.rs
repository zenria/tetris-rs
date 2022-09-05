use bevy::prelude::Component;

pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_HEIGHT: u32 = 25;

///Position in the Board
#[derive(Component)]
pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl BoardPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
