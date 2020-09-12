use super::Color::Color;
use super::Piece::{Piece, PieceType};
use std::collections::HashMap;

//mod SpecialMoves;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Space(i32, i32);

impl Space {
    pub fn is_in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }
    pub fn offset(&self, rank_offset: i32, file_offset: i32) -> Self {
        Self(self.0 + rank_offset, self.1 + file_offset)
    }
    pub fn new(rank: i32, file: i32) -> Self {
        Self(rank, file)
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

    fn state_from_move(&self, from: Space, to: Space) -> Self {
        assert!(from.is_in_bounds());
        assert!(to.is_in_bounds());
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

        new_state.to_play = new_state.to_play.inverse();
        new_state
    }

    fn get_full_action_space(&self) -> HashMap<Space, HashMap<Space, GameState>> {
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
        let move_pattern = piece.get_move_pattern();
        let mut action_space: HashMap<Space, GameState> = HashMap::new();
        if piece.piece_type != PieceType::Pawn { 
        for dir in move_pattern {
            let mut to = from.clone();
            loop {
                to = to.offset(dir.0, dir.1);
                if !to.is_in_bounds() {
                    break;
                }
                if !self.board.contains_key(&to) {
                    action_space.insert(to, self.state_from_move(from, to));
                } else if self.board[&to].color == piece.color.inverse() {
                    action_space.insert(to, self.state_from_move(from, to));
                    break;
                } else if self.board[&to].color == piece.color {
                    break;
                }
            }
        }
    } else {
        //action_space = get_pawn_moves();
    }
        //SPECIAL MOVES HERE
        if piece.piece_type == PieceType::King{
            //action_space.append(get_casteling_moves());
        }
        action_space
    }
}

#[cfg(test)]
mod gamestate {
    use super::*;
    #[test]
    fn board_test() {
        let s = Space(2, 5);
        let p = Piece::new(Color::White, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(s, p);
        assert_eq!(p, g.board[&s]);
    }
    #[test]
    fn get_standard_move() {
        let p = Piece::new(Color::White, PieceType::Rook);
        let mp: Vec<(i32, i32, bool)> = p.get_move_pattern();
        assert_eq!(mp.len(), 4);
    }

    #[test]
    fn piece_action_space() {
        let s = Space(5, 5);
        let p = Piece::new(Color::White, PieceType::Rook);
        let mut g = GameState::new_blank();
        g.board.insert(s, p);
        let p_as = g.get_piece_action_space(s);

        assert!(p_as.contains_key(&Space(6, 5)));
        assert!(p_as.contains_key(&Space(5, 4)));
        assert!(!p_as.contains_key(&Space(6, 6)));
        assert!(!p_as.contains_key(&Space(-1, 5)));
        assert!(!p_as.contains_key(&Space(5, 8)));
    }
    #[test]
    fn piece_action_space_blocking() {
        let white_rook = Piece::new(Color::White, PieceType::Rook);
        let white_pawn = Piece::new(Color::White, PieceType::Pawn);
        let black_knight = Piece::new(Color::Black, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(Space(5, 5), white_rook);
        g.board.insert(Space(5, 6), white_pawn);
        g.board.insert(Space(3, 5), black_knight);
        let p_as = g.get_piece_action_space(Space(5, 5));

        assert!(p_as.contains_key(&Space(7, 5)));
        assert!(p_as.contains_key(&Space(5, 0)));
        assert!(!p_as.contains_key(&Space(5, 6)));
        assert!(!p_as.contains_key(&Space(5, 7)));
        assert!(p_as.contains_key(&Space(3, 5)));
        assert!(!p_as.contains_key(&Space(2, 5)));
    }

    //Space tests
    #[test]
    fn out_of_bounds_space() {
        assert!(!Space(-1, 3).is_in_bounds());
        assert!(!Space(0, 8).is_in_bounds());
    }
    #[test]
    fn space_eq() {
        assert_eq!(Space(1, 1), Space(1, 1));
    }
    #[test]
    fn space_offset() {
        assert_eq!(Space(4, 7), Space(1, 2).offset(3, 5));
    }
    #[test]
    fn constructor() {
        let s = Space::new(2, 3);
        assert_eq!(s, Space(2, 3))
    }

    #[test]
    fn to_play(){
        let white_rook = Piece::new(Color::White, PieceType::Rook);
        let white_pawn = Piece::new(Color::White, PieceType::Pawn);
        let black_knight = Piece::new(Color::Black, PieceType::Knight);
        let mut g = GameState::new_blank();
        g.board.insert(Space(5, 5), white_rook);
        g.board.insert(Space(5, 6), white_pawn);
        g.board.insert(Space(3, 5), black_knight);

        let g2 = g.state_from_move(Space(5,5), Space(4,5));
        assert_ne!(g2.to_play, g.to_play);
    }
}
