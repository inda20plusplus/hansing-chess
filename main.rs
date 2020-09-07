
/*
TODO:
 *  PIECE
 *  ==============
 *  COLORS / TEAMS
 *      F: Identity - T: white == white
 *      F: Inverse - T: inverse(white) == black
 *  PIECE TYPES
 *      F: Identity - T: rook == rook
 *  MOVES
 *      F: Diagonal moves - T: -
 *      F: Orthogonal moves - T: -
 *      F: Knights moves - T: -
 *      F: Pawns move - T: -
 *      F: Pawns capture - T: -
 *      F: Promotion move - T: -
 *      F: En passant - T: -
 *      F: Kings move - T: -
 *      F: Castling - T: -
 *
 *  MOVEMENT
 *  ===================
 *  CHECK MOVE LEGALITY
 *      F: Prevent self check.
 *
 *
 *  CHECK CHESS
 *  BOARD
 *  =====
 *      F: "Nullable" - T: --------------------
 *      F: Check legal - T: deny call of (0,8) / (-1, 1) / (0, 8) / (5, -1)
 *
 *
 */

use std::fmt;

fn main(){
test_bishop_moves()
}

#[derive(PartialEq, Copy, Clone)]
enum Color{
    White,
    Black
}

use Color::White;
use Color::Black;

impl Color{
    fn inverse(&self) -> Color{
        match self {
            White => Black,
            Black => White
        }
    }

    fn forwards(&self) -> i8 { //is i8 appropriate?
        match self {
            White => 1,
            Black => -1
        }
    }
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            White => write!(f, "White"),
            Black => write!(f, "Black")
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

use PieceType::Pawn;
use PieceType::Knight;
use PieceType::Bishop;
use PieceType::Rook;
use PieceType::Queen;
use PieceType::King;
use std::any::Any;

impl fmt::Display for PieceType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Pawn => write!(f, "Pawn"),
            Knight=>write!(f, "Knight"),
            Bishop=>write!(f, "Bishop"),
            Rook=>write!(f, "Rook"),
            Queen=>write!(f, "Queen"),
            King=>write!(f, "King")
        }
    }
}

impl Piece{
    fn get_standard_movement_rules(&self) -> Vec<(i8, i8, bool)>{
        match self.piece_type {
            Pawn    => vec![(self.color.forwards(), 0, false)],

            Knight  => vec![(2, 1, false), (2, -1, false),
                            (-2, 1, false), (-2, -1, false),
                            (1, 2, false), (-1, 2, false),
                            (1, -2, false), (-1, -2, false)],

            Bishop  => vec![(1, 1, true), (-1, -1, true),
                            (-1, 1, true), (1, -1, true)],

            Rook    => vec![(1, 0, true), (-1, 0, true),
                            (0, 1, true), (0, -1, true)],
            Queen   => vec![(1, 0, true), (-1, 0, true),
                            (0, 1, true), (0, -1, true),
                            (1, 1, true), (-1, -1, true),
                            (-1, 1, true), (1, -1, true)],

            King    => vec![(1, 0, false), (-1, 0, false),
                            (0, 1, false), (0, -1, false),
                            (1, 1, false), (-1, -1, false),
                            (-1, 1, false), (1, -1, false)]
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Self{
        Self {
            color,
            piece_type,
        }
    }
}

struct MoveData{
    from: (usize, usize),
    to: (usize, usize),
}

impl MoveData{
    fn new(from: (usize, usize), to: (usize, usize)) -> Self{
        Self{
            from,
            to
        }
    }
}

impl fmt::Display for MoveData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!("{}{}")
    }
}

struct BoardState {
    board: [[Option<Piece>; 8]; 8],
    prisoners: Vec<Piece>,
    to_play: Color,
}

impl BoardState {
    fn new_blank() -> Self{
        Self {
            board: [[None; 8]; 8],
            prisoners: Vec::new(),
            to_play: White
        }
    }
    fn check_standard_move_legality(&self, to: (i8, i8), piece: &Piece) -> bool{
        if !is_in_bounds(to) {
            return false;
        }
        let to_content = self.board[to.0 as usize][to.1 as usize];
        if to_content.is_some() && (to_content.unwrap().color == piece.color || piece.piece_type == Pawn){
            return false;
        }
        true
    }
    fn get_action_space(){

    }
    fn get_piece_action_space(&self, from: (usize, usize), piece: &Piece) -> Vec<MoveData>{ //MIGHT NEED TO BE OPTION
    //Rook movement
        let mut action_space = Vec::new();
        let standard_move_rules = piece.get_standard_movement_rules();
        for sm in standard_move_rules{ //TODO consider names
            //let mut to: (i8, i8) = (from.0 + sm.0, from.1 + sm.1);
            for i in 1..8i8 {
                let to: (i8, i8) = (from.0 as i8 + sm.0 * i, from.1 as i8 + sm.1 * i);
                if self.check_standard_move_legality(to, piece) { break; }
                action_space.push(MoveData::new(from, (to.0 as usize, to.1 as usize)));
                if !sm.2 { break; }
            }
        }
        action_space
    }
}


fn is_in_bounds(space: (i8, i8)) -> bool{
    space.0 >= 0 &&
        space.0 < 8 &&
        space.1 >= 0 &&
        space.1 < 8
}



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_color(){

    //COLOR EQUALS
    assert!(White == White);
    assert!(White != Black);
    //COLOR INVERSE
    assert!(White.inverse() == Black);
    assert!(Black.inverse() == White);
    let w = White;
    assert!(w.inverse() == Black);
    assert!(w.inverse() == Black);
    //COLOR DISPLAY
    assert_eq!(format!("{}", White), "White");
    assert_eq!(format!("{}", Black), "Black");
    }

    #[test]
    fn test_piece_type() {
        // TYPE EQUALS
        assert!(Pawn == Pawn);
        assert!(Bishop != Knight);
        // TYPE DISPLAY
        assert_eq!(format!("{}", Knight), "Knight");
        assert_eq!(format!("{}", King), "King");
    }

    #[test]
    fn test_board(){
        let mut blank_board = BoardState::new_blank();
        assert!(blank_board.board[4][5].is_none());
        blank_board.prisoners.push(Piece::new(White, Queen));
        assert!(blank_board.prisoners[0].color == White);

        //blank_board.board[0][0] == Piece::new(Black, King); //SO FAR: NO NEED FOR PIECE EQUALS

    }
    #[test]
    #[should_panic]
    fn test_board_oob(){
        let mut blank_board = BoardState::new_blank();
        let out_of_bounds = blank_board.board[0][9];
    }
    #[test]
    fn test_bishop_moves(){
        let mut board = BoardState::new_blank();
        board.board[5][5] = Some(Piece::new(White, Bishop));
        assert!(board.get_piece_action_space((5,5),board.board[5][5].unwrap()));

    }

}
fn test_bishop_moves(){
        let mut board = BoardState::new_blank();
        board.board[5][5] = Some(Piece::new(White, Bishop));
        //assert!(board.get_piece_action_space((5,5),board.board[5][5].unwrap()));
    for i in board.get_piece_action_space((5,5),&board.board[5][5].unwrap()) {
        println!("{:?}", i)
    }

    }

