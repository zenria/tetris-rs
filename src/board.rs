use bevy::prelude::Component;

pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 25;

///Position in the Board
#[derive(Component)]
pub struct BoardPosition {
    pub x: i32,
    pub y: i32,
}

impl BoardPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
