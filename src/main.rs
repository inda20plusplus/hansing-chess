use std::collections::HashMap;
use std::ops::Deref;
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
            has_moved: false,
        }
    }

    fn get_move_pattern(&self) -> Vec<(i32, i32, bool)> {
        match self.piece_type {
            PieceType::Pawn => vec![(self.color.forward(), 0, false)],

            PieceType::Knight => vec![
                (2, 1, false),
                (2, -1, false),
                (-2, 1, false),
                (-2, -1, false),
                (1, 2, false),
                (-1, 2, false),
                (1, -2, false),
                (-1, -2, false),
            ],

            PieceType::Bishop => vec![(1, 1, true), (-1, -1, true), (-1, 1, true), (1, -1, true)],

            PieceType::Rook => vec![
                (1, 0, true),
                (-1, 0, true),
                (0, 1, true),
                (0, -1, true)],
            PieceType::Queen => vec![
                (1, 0, true),
                (-1, 0, true),
                (0, 1, true),
                (0, -1, true),
                (1, 1, true),
                (-1, -1, true),
                (-1, 1, true),
                (1, -1, true),
            ],

            PieceType::King => vec![
                (1, 0, false),
                (-1, 0, false),
                (0, 1, false),
                (0, -1, false),
                (1, 1, false),
                (-1, -1, false),
                (-1, 1, false),
                (1, -1, false),
            ]
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
#[derive(Clone)]
struct GameState {
    board: HashMap<Space, Piece>,
    captured: Vec<Piece>,
    to_play: Color,
}

impl GameState {
    fn new_blank() -> Self {
        Self {
            board: HashMap::new(),
            captured: Vec::new(),
            to_play: Color::White,
        }
    }

    fn state_from_move(&self, from: Space, to: Space) -> Self{
        //PANIC! IF OUT OF BOUNDS
        let mut new_state = self.clone();
        
        if new_state.board.contains_key(&to) {
            let capture = new_state.board[&to];
            new_state.captured.push(capture);
        }
        let mut piece = new_state.board[&from];
        piece.has_moved = true;
        new_state.board.remove(&to);
        new_state.board.remove(&from);
        new_state.board.insert(to, piece);
        new_state
    }

    fn get_full_action_space(&self) -> HashMap<Space, HashMap<Space, GameState>>{
        let mut action_space: HashMap<Space, HashMap<Space, GameState>> = HashMap::new();
        for (s, p) in self.board.iter() {
            if p.color == self.to_play {
                action_space.insert(*s, self.get_piece_action_space(*s));
            }
        }
        action_space
    }
    
    fn get_piece_action_space(&self, from: Space) -> HashMap<Space, GameState> {
        let piece = self.board[&from];
        println!("{:?}",piece);
        let move_pattern = piece.get_move_pattern();
        println!(">{:?}",move_pattern);
        let mut action_space: HashMap<Space, GameState> = HashMap::new();
        for dir in move_pattern {
            let mut to = from.clone();
            loop {
                to = to.offset(dir.0, dir.1);
                if !to.is_in_bounds(){ break;}
                if !self.board.contains_key(&to){
                    action_space.insert(to, self.state_from_move(from, to));
                } else if self.board[&to].color == piece.color.inverse() {
                    action_space.insert(to, self.state_from_move(from, to));
                    break;
                } else if self.board[&to].color == piece.color {
                    break;
                }
            }
        }
        //SPECIAL MOVES HERE
        action_space
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
    fn board_test() {
        let s = Space(2, 5);
        let p = Piece::new(Color::White, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(s, p);
        assert_eq!(p, g.board[&s]);
    }
    #[test]
    fn get_standard_move(){
        let p = Piece::new(Color::White, PieceType::Rook);
        let mp: Vec<(i32,i32,bool)> = p.get_move_pattern();
        assert_eq!(mp.len(), 4);
    }

    #[test]
    fn piece_action_space() {
        let s = Space(5, 5);
        let p = Piece::new(Color::White, PieceType::Rook);
        let mut g = GameState::new_blank();
        g.board.insert(s, p);
        let p_as = g.get_piece_action_space(s);

        assert!(p_as.contains_key(&Space(6,5)));
        assert!(p_as.contains_key(&Space(5,4)));
        assert!(!p_as.contains_key(&Space(6,6)));
        assert!(!p_as.contains_key(&Space(-1,5)));
        assert!(!p_as.contains_key(&Space(5,8)));
    }
    #[test]
    fn piece_action_space_blocking() {
        
        let white_rook = Piece::new(Color::White, PieceType::Rook);
        let white_pawn = Piece::new(Color::White, PieceType::Pawn);
        let black_knight = Piece::new(Color::Black, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(Space(5,5), white_rook);
        g.board.insert(Space(5,6), white_pawn);
        g.board.insert(Space(3,5), black_knight);
        let p_as = g.get_piece_action_space(Space(5,5));

        assert!(p_as.contains_key(&Space(7,5)));
        assert!(p_as.contains_key(&Space(5,0)));
        assert!(!p_as.contains_key(&Space(5,6)));
        assert!(!p_as.contains_key(&Space(5,7)));
        assert!(p_as.contains_key(&Space(3,5)));
        assert!(!p_as.contains_key(&Space(2,5)));
    }

    
}

fn main() {
    let white_rook = Piece::new(Color::White, PieceType::Rook);
        let white_pawn = Piece::new(Color::White, PieceType::Pawn);
        let black_knight = Piece::new(Color::Black, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(Space(5,5), white_rook);
        g.board.insert(Space(5,6), white_pawn);
        g.board.insert(Space(3,5), black_knight);
        let p_as = g.get_full_action_space();
        for (a, b) in p_as.iter(){
            for (c, d) in p_as.iter(){
                println!("{:?}->{:?}",a, c);
            }
            println!();
        }


}
