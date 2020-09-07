/* TODO!
 * =====
 *      Color
 *      PieceType
 *      BoardState
 *  move_execute
 *  MoveData
 *  standard_move_rules
 *  get_action_space_from
 *
 */
use std::fmt;
#[derive(PartialEq, Copy, Clone)]
enum Color {
    White,
    Black,
}

use Color::{White, Black};

impl Color {
    fn inverse(c: Color) -> Self {
        match c {
            White => Black,
            Black => White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            White => write!(f, "White"),
            Black => write!(f, "Black"),
        }
    }
}
#[derive(PartialEq, Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

use PieceType::{Pawn, Knight, Bishop, Rook, Queen, King};

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pawn => write!(f, "Pawn"),
            Knight => write!(f, "Knight"),
            Bishop => write!(f, "Bishop"),
            Rook => write!(f, "Rook"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King"),
        }
    }
}
#[derive(Copy, Clone)]
struct Piece{
    color: Color,
    p_type: PieceType,
}

#[derive(Clone)]
struct GameState{
    board: [[Option<Piece>; 8]; 8],
    prisoners: Vec<Piece>,
}

impl GameState{
    fn new_blank() -> Self {
        Self {
            board: Default::default(),
            prisoners: Vec::new(),
        }
    }

    fn execute_move(&self, from_rank: usize, from_file: usize, to_rank: usize, to_file: usize) -> Self {
        let mut new_state = self.clone();
        let capture = new_state.board[to_rank][to_file];
        if capture.is_some(){
            new_state.prisoners.push(capture.unwrap());
        }
        new_state.board[to_rank][to_file] = new_state.board[from_rank][from_file];
        new_state.board[from_rank][from_file] = None;
        new_state
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_color_eq() {
        assert!(White == White);
        assert!(Black == Black);
    }
    #[test]
    fn test_color_disp() {
        assert_eq!(format!("{}", White), "White");
        assert_eq!(format!("{}", Black), "Black");
    }

    #[test]
    fn test_piece_type_eq() {
        assert!(PieceType::Bishop == PieceType::Bishop);
    }
    #[test]
    fn test_piece_type_disp() {
        assert_eq!(format!("{}", Pawn), "Pawn");
    }
    #[test]
    fn test_game_state(){
        let s = GameState::new_blank();

    }
}

fn main() {
    println!("Hello, chess!");
}
