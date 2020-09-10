#[derive(PartialEq, Debug, Copy, Clone)]
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
}

#[cfg(test)]
mod test_color{
    use super::*;
    #[test]
    fn equals() {
        assert_eq!(Color::White, Color::White);
    }
    #[test]
    fn inverse() {
        assert_eq!(Color::White.inverse(), Color::Black);
    }
    #[test]
    fn forward(){
        assert_eq!(Color::White.forward(), 1);
        assert_eq!(Color::Black.forward(), -1);
    }
}