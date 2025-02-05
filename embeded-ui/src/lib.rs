#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod ui;
use core::ops::{Add, AddAssign, Sub, SubAssign};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub relative: Position,
    pub offset: Position,
}

impl Coord {
    /// Get the absolute position of the [Coord]
    pub fn get_absolute_pos(&self) -> Position {
        self.relative + self.offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: i16,
    pub height: i16,
}

impl Size {
    pub fn new(width: i16, height: i16) -> Self {
        Self { width, height }
    }

    /// Create a new [Size] with the width and height being the width and height of a rectangle
    /// with the two [Positions](Position) beinng oposite corners
    ///
    /// ```
    /// # use embeded_ui::{Position, Size};
    /// let a = Position::new(0,0);
    /// let b = Position::new(5,3);
    /// assert_eq!(Size{width:5,height:3}, Size::size_between(a,b));
    /// ```
    pub fn size_between(a: Position, b: Position) -> Self {
        let diff = b - a;
        Self {
            width: diff.x.abs(),
            height: diff.y.abs(),
        }
    }
}
