#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn inverse(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
    pub fn forward(&self) -> i32 {
        match self {
            Color::Black => -1,
            Color::White => 1,
        }
    }
    pub fn seventh_rank(&self) -> i32 {
        match self {
            Color::Black => 1,
            Color::White => 6,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

#[cfg(test)]
mod test_color {
    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Color::White, Color::White)
    }
    #[test]
    fn inverse() {
        assert_eq!(Color::White.inverse(), Color::Black)
    }
    #[test]
    fn seventh_rank() {
        assert_eq!(Color::Black.seventh_rank(), 1)
    }
    #[test]
    fn index() {
        let arr = [10, 20];
        assert_eq!(arr[Color::White.index()], 10);
        assert_eq!(arr[Color::Black.index()], 20)
    }
}
