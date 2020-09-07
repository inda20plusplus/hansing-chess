use std::collections::HashMap;
use std::fmt::Debug;
#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    White,
    Black,
}

impl Color {
    fn inverse(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
    fn forward(&self) -> i32 {
        match self {
            Color::Black => -1,
            Color::White => 1,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(PartialEq, Debug, Copy, Clone)]
struct Piece {
    color: Color,
    piece_type: PieceType,
    has_moved: bool,
}


impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Self {
        Self {
            color,
            piece_type,
            has_moved: true,
        }
    }

    fn get_move_pattern(&self) -> Vec<(i32, i32, bool)> {
        //////////////////////////////////////////WHY DOSE IT NOT WORK!!==!=??!?!?!?!?!
        match self.piece_type {
            Pawn => vec![(self.color.forward(), 0, false)],

            Knight => vec![
                (2, 1, false),
                (2, -1, false),
                (-2, 1, false),
                (-2, -1, false),
                (1, 2, false),
                (-1, 2, false),
                (1, -2, false),
                (-1, -2, false),
            ],

            Bishop => vec![(1, 1, true), (-1, -1, true), (-1, 1, true), (1, -1, true)],

            Rook => vec![(1, 0, true), (-1, 0, true), (0, 1, true), (0, -1, true)],
            Queen => vec![
                (1, 0, true),
                (-1, 0, true),
                (0, 1, true),
                (0, -1, true),
                (1, 1, true),
                (-1, -1, true),
                (-1, 1, true),
                (1, -1, true),
            ],

            King => vec![
                (1, 0, false),
                (-1, 0, false),
                (0, 1, false),
                (0, -1, false),
                (1, 1, false),
                (-1, -1, false),
                (-1, 1, false),
                (1, -1, false),
            ],
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Space(i32, i32);

impl Space {
    fn is_in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }
    fn offset(&self, rank_offset: i32, file_offset: i32) -> Self {
        Self(self.0 + rank_offset, self.1 + file_offset)
    }
}

struct GameState {
    board: HashMap<Space, Piece>,
    captured: Vec<Piece>,
}

impl GameState {
    fn new_blank() -> Self {
        Self {
            board: HashMap::new(),
            captured: Vec::new(),
        }
    }

    fn get_piece_actionspace(&self, space: &Space) {
        // WHY "&" HERE??????
        let piece = self.board[space];
    }
}

/* TESTS
 * =====
 * Color
 * -inverse
 * PieceType
 * Piece
 * Space
 * -check_in_bounds
 * GameState
 * -Board
 * -Captured
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn color_equals() {
        assert_eq!(Color::White, Color::White);
    }
    #[test]
    fn color_inverse() {
        assert_eq!(Color::White.inverse(), Color::Black);
    }
    #[test]
    fn piece_type_equals() {
        assert_eq!(PieceType::Bishop, PieceType::Bishop);
    }
    #[test]
    fn out_of_bounds_space() {
        assert!(!Space(-1, 3).is_in_bounds());
    }
    #[test]
    fn space_eq() {
        assert_eq!(Space(1, 1), Space(1, 1));
    }
    #[test]
    fn space_offset() {
        assert_eq!(Space(4, 6), Space(1, 1).offset(3, 5));
    }
    #[test]
    fn board_test(){
        let s = Space(2,5);
        let p = Piece::new(Color::White, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(s, p);
        assert_eq!(p, g.board[&s]);
    }
}

fn main() {
    println!()
}
