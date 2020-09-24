use crate::color::Color;
use crate::title::Title;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub title: Title,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, title: Title) -> Self {
        Self {
            color,
            title,
            has_moved: false,
        }
    }

    pub fn to_char(&self) -> char {
        if self.color == Color::White {
            match self.title {
                Title::Pawn => 'P',
                Title::Knight => 'N',
                Title::Rook => 'R',
                Title::Bishop => 'B',
                Title::Queen => 'Q',
                Title::King => 'K',
            }
        } else {
            match self.title {
                Title::Pawn => 'p',
                Title::Knight => 'n',
                Title::Rook => 'r',
                Title::Bishop => 'b',
                Title::Queen => 'q',
                Title::King => 'k',
            }
        }
    }
}
use std::fmt;
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.color {
            Color::White => "White",
            Color::Black => "Black",
        };
        let t = match self.title {
            Title::Pawn => "Pawn",
            Title::Knight => "Knight",
            Title::Rook => "Rook",
            Title::Bishop => "Bishop",
            Title::Queen => "Queen",
            Title::King => "King",
        };

        write!(f, "{} {}", c, t)
    }
}

#[cfg(test)]
mod test_piece {
    use super::*;
    #[test]
    fn eq() {
        let p1 = Piece::new(Color::Black, Title::Rook);
        let p2 = p1.clone();
        assert_eq!(p1, p2)
    }
}
