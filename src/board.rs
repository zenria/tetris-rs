use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use bevy::prelude::Component;

pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 25;

/// Position in the Board
///
/// Each square that are composing pieces have a BoardPosition
/// component to ease computing
#[derive(Component, Clone, Copy, Debug)]
pub struct BoardPosition {
    pub x: i32,
    pub y: i32,
}

impl Add<BoardPosition> for (i32, i32) {
    type Output = Self;

    fn add(self, rhs: BoardPosition) -> Self::Output {
        (self.0 + rhs.x, self.1 + rhs.y)
    }
}
impl Sub<BoardPosition> for (i32, i32) {
    type Output = Self;

    fn sub(self, rhs: BoardPosition) -> Self::Output {
        (self.0 - rhs.x, self.1 - rhs.y)
    }
}

impl Add<(i32, i32)> for BoardPosition {
    type Output = Self;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        BoardPosition {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}
impl Sub<(i32, i32)> for BoardPosition {
    type Output = Self;
    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        BoardPosition {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl BoardPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn into_idx(self) -> usize {
        assert!(self.x >= 1);
        assert!(self.y >= 1);
        assert!(self.x <= BOARD_WIDTH);
        assert!(self.y <= BOARD_HEIGHT);

        (self.x - 1 + (self.y - 1) * BOARD_WIDTH) as usize
    }
}

impl From<(i32, i32)> for BoardPosition {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

/// Helper struct  that detects complete lines
pub struct Board {
    inner: [bool; BOARD_WIDTH as usize * BOARD_HEIGHT as usize],
}

const FULL_LINE: [bool; BOARD_WIDTH as usize] = [true; BOARD_WIDTH as usize];

impl Board {
    pub fn is_line_full(&self, line: i32) -> bool {
        let start_pos = BoardPosition::new(1, line);
        let end_post = BoardPosition::new(BOARD_WIDTH, line);
        self.inner[start_pos.into_idx()..=end_post.into_idx()] == FULL_LINE
    }

    /// is in a wall or in a fixed square?
    pub fn is_concrete<BP: Into<BoardPosition>>(&self, bp: BP) -> bool {
        let bp: BoardPosition = bp.into();
        if bp.x == 0 || bp.x == BOARD_WIDTH + 1 || bp.y == 0 {
            // wall
            true
        } else if bp.y > BOARD_HEIGHT {
            // outide this board
            false
        } else {
            self.inner[bp.into_idx()]
        }
    }
}

impl FromIterator<BoardPosition> for Board {
    fn from_iter<T: IntoIterator<Item = BoardPosition>>(iter: T) -> Self {
        let mut ret = Board {
            inner: [false; BOARD_WIDTH as usize * BOARD_HEIGHT as usize],
        };
        for bp in iter {
            if bp.x > 0 && bp.y > 0 && bp.x <= BOARD_WIDTH {
                ret.inner[bp.into_idx()] = true;
            }
        }
        ret
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (1..BOARD_HEIGHT).rev() {
            for x in 1..=BOARD_WIDTH {
                let bp = BoardPosition::new(x, y);
                if self.inner[bp.into_idx()] {
                    f.write_str("*")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
