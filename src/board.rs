use crate::color::Color;
use crate::movedata::{MoveData, MoveType};
use crate::moverules::*;
use crate::occupancy::Occupancy;
use crate::piece::Piece;
use crate::square::Square;
use crate::standardstart::standard_setup;
use crate::title::Title;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Board {
    pub pieces: HashMap<Square, Piece>,
    pub captures: Vec<Piece>,
    pub to_act: Color,
    pub in_check: bool,        //Current player
    pub blockers: Vec<Square>, //Current player
    pub king_pos: [Square; 2], //Indexed White, Black
    pub en_passant: Option<Square>,
}

impl Board {
    fn new_empty() -> Self {
        Self {
            pieces: HashMap::with_capacity(32),
            captures: Vec::with_capacity(32),
            to_act: Color::White,
            in_check: false,
            blockers: Vec::with_capacity(8),
            king_pos: [Square::new(0, 4).unwrap(), Square::new(7, 4).unwrap()],
            en_passant: None,
        }
    }
    pub fn new_standard() -> Self {
        let mut board = Self::new_empty();
        standard_setup(&mut board.pieces);
        board
    }

    pub fn occupancy(&self, s: Square, c: Color) -> Occupancy {
        if self.pieces.contains_key(&s) {
            if self.pieces[&s].color == c {
                Occupancy::SameColor
            } else {
                Occupancy::OtherColor
            }
        } else {
            Occupancy::Empty
        }
    }
    fn update_king_pos(&mut self) {
        for &c in [Color::White, Color::Black].iter() {
            if self.pieces.contains_key(&self.king_pos[c.index()]) {
                if self.pieces[&self.king_pos[c.index()]].title == Title::King
                    && self.pieces[&self.king_pos[c.index()]].color == c
                {
                    continue;
                }
            }
            for (s, p) in self.pieces.iter() {
                if p.color == c && p.title == Title::King {
                    self.king_pos[c.index()] = *s;
                }
            }
        }
    }
    pub fn next_turn(&mut self) {
        self.to_act = self.to_act.inverse();
        self.update_king_pos();
        //check for check and blockers
        let (c, b) = self.check_if_threatend(self.king_pos[self.to_act.index()], self.to_act);
        self.in_check = c;
        self.blockers = b;
    }
    fn remove_piece(&mut self, s: Square) -> Option<Piece> {
        self.pieces.remove(&s)
    }
    fn replace_piece(&mut self, s: Square, p: Piece) -> Option<Piece> {
        self.pieces.insert(s, p)
    }
    fn move_piece(&mut self, from: Square, to: Square) -> Option<Piece> {
        let p = self.remove_piece(from);
        if let Some(mut p) = p {
            p.has_moved = true;
            self.replace_piece(to, p)
        } else {
            None
        }
    }
    fn remove_piece_with_capture(&mut self, s: Square) {
        let c = self.remove_piece(s);
        if let Some(c) = c {
            self.captures.push(c);
        }
    }
    fn replace_piece_with_capture(&mut self, s: Square, p: Piece) {
        let c = self.replace_piece(s, p);
        if let Some(c) = c {
            self.captures.push(c);
        }
    }
    fn move_piece_with_capture(&mut self, from: Square, to: Square) {
        let p = self.remove_piece(from);
        if let Some(mut p) = p {
            p.has_moved = true;
            self.replace_piece_with_capture(to, p);
        }
    }

    pub fn make_move(&mut self, move_data: MoveData) {
        self.en_passant = None;
        match move_data.move_type {
            MoveType::Standard => self.make_standard_move(move_data),
            MoveType::DoubleStep(en_passant_oppertunity) => {
                self.make_double_step_move(move_data, en_passant_oppertunity)
            }
            MoveType::Casteling(rook_from, rook_to) => {
                self.make_casteling_move(move_data, rook_from, rook_to)
            }
            MoveType::EnPassant(capture) => self.make_en_passant_move(move_data, capture),
            MoveType::Promotion(promote_to) => self.make_promotion_move(move_data, promote_to),
            //argo _ => panic!("Invalid move type!"),
        }
        self.next_turn();
    }

    fn make_standard_move(&mut self, move_data: MoveData) {
        self.move_piece_with_capture(move_data.from, move_data.to);
    }

    fn make_double_step_move(&mut self, move_data: MoveData, en_passant_oppertunity: Square) {
        self.move_piece(move_data.from, move_data.to);
        self.en_passant = Some(en_passant_oppertunity);
    }

    fn make_casteling_move(&mut self, move_data: MoveData, rook_from: Square, rook_to: Square) {
        self.move_piece(move_data.from, move_data.to);
        self.move_piece(rook_from, rook_to);
    }

    fn make_en_passant_move(&mut self, move_data: MoveData, capture: Square) {
        self.move_piece(move_data.from, move_data.to);
        self.remove_piece_with_capture(capture);
    }

    fn make_promotion_move(&mut self, move_data: MoveData, promote_to: Option<Title>) {
        self.move_piece_with_capture(move_data.from, move_data.to);
        if let Some(promotion_piece) = promote_to {
            self.replace_piece(
                move_data.to,
                Piece::new(self.pieces[&move_data.to].color, promotion_piece),
            );
        } else {
            self.replace_piece(
                move_data.to,
                Piece::new(self.pieces[&move_data.to].color, Title::Queen),
            );
        }
    }

    pub fn check_if_threatend(&self, to: Square, color: Color) -> (bool, Vec<Square>) {
        let mut threatend = false;
        let mut blockers = vec![to];
        // Knight threats
        for (rank_offset, file_offset, _) in KNIGHT_MOVES.to_vec() {
            if let Some(from) = to.offset(rank_offset, file_offset) {
                if self.occupancy(from, color) == Occupancy::OtherColor {
                    if self.pieces[&from].title == Title::Knight {
                        threatend = true;
                    }
                }
            }
        }
        //Bishop threats
        for (rank_offset, file_offset, _) in BISHOP_MOVES.to_vec() {
            let mut from = to;
            loop {
                if let Some(temp) = from.offset(rank_offset, file_offset) {
                    from = temp;

                    if self.occupancy(from, color) == Occupancy::OtherColor {
                        blockers.push(from);

                        if self.pieces[&from].title == Title::Bishop
                            || self.pieces[&from].title == Title::Queen
                        {
                            threatend = true;
                        }

                        break;
                    } else if self.occupancy(from, color) == Occupancy::SameColor {
                        blockers.push(from);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        //Rook threats
        for (rank_offset, file_offset, _) in ROOK_MOVES.to_vec() {
            let mut from = to;
            loop {
                if let Some(temp) = from.offset(rank_offset, file_offset) {
                    from = temp;

                    if self.occupancy(from, color) == Occupancy::OtherColor {
                        blockers.push(from);

                        if self.pieces[&from].title == Title::Rook
                            || self.pieces[&from].title == Title::Queen
                        {
                            threatend = true;
                        }

                        break;
                    } else if self.occupancy(from, color) == Occupancy::SameColor {
                        blockers.push(from);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        //Pawn threats
        for &file_offset in [1, -1].iter() {
            if let Some(from) = to.offset(color.forward(), file_offset) {
                if self.occupancy(from, color) == Occupancy::OtherColor {
                    if self.pieces[&from].title == Title::Pawn {
                        threatend = true
                    }
                }
            }
        }
        (threatend, blockers)
    }
}

#[cfg(test)]
mod test_board {
    use super::*;
    #[test]
    fn test_threatend_knight() {
        let mut board = Board::new_standard();

        let b8 = Square::new(7, 1).unwrap();
        let e4 = Square::new(3, 4).unwrap();
        let e1 = Square::new(0, 4).unwrap();
        let g3 = Square::new(2, 6).unwrap();
        let e2 = Square::new(1, 5).unwrap();
        let move1 = MoveData::new_standard(e1, g3);
        let move2 = MoveData::new_standard(b8, e4);

        board.make_move(move1);
        let (a, _) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(!a);
        //assert!(b.contains(&g2));
        board.make_move(move2);
        let (a, b) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(a);
        assert!(b.contains(&e2));
    }
    #[test]
    fn test_threatend_bishop() {
        let mut board = Board::new_standard();

        let c8 = Square::new(7, 2).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let e1 = Square::new(0, 4).unwrap();
        let g3 = Square::new(2, 6).unwrap();
        let e2 = Square::new(1, 5).unwrap();
        let move1 = MoveData::new_standard(e1, g3);
        let move2 = MoveData::new_standard(c8, e5);

        board.make_move(move1);
        let (a, _) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(!a);
        //assert!(b.contains(&g2));
        board.make_move(move2);
        let (a, b) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(a);
        assert!(b.contains(&e2));
    }
    #[test]
    fn test_threatend_rook() {
        let mut board = Board::new_standard();

        let a8 = Square::new(7, 0).unwrap();
        let e3 = Square::new(2, 4).unwrap();
        let e1 = Square::new(0, 4).unwrap();
        let g3 = Square::new(2, 6).unwrap();
        let g2 = Square::new(1, 6).unwrap();
        let g4 = Square::new(3, 6).unwrap();
        let move1 = MoveData::new_standard(e1, g3);
        let move2 = MoveData::new_standard(a8, e3);

        board.make_move(move1);
        let (a, _) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(!a);
        //assert!(b.contains(&g2));
        board.make_move(move2);
        let (a, b) = board.check_if_threatend(g3, board.pieces[&g3].color);
        assert!(a);
        assert!(b.contains(&g2));
        assert!(b.contains(&e3));
        assert!(!b.contains(&g4));
    }
    #[test]
    fn test_threatend_pawn() {
        let mut board = Board::new_standard();

        let e1 = Square::new(0, 4).unwrap();
        let g6 = Square::new(5, 6).unwrap();

        let move1 = MoveData::new_standard(e1, g6);
        //let move2= MoveData::new_standard(a8, e3);
        let (a, _) = board.check_if_threatend(e1, board.pieces[&e1].color);

        assert!(!a);
        board.make_move(move1);
        let (a, _) = board.check_if_threatend(g6, board.pieces[&g6].color);
        assert!(a);
    }
    #[test]
    fn make_move_from_move_data() {
        let d2 = Square::new(1, 3).unwrap();
        let d5 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let e6 = Square::new(5, 4).unwrap();
        let e7 = Square::new(6, 4).unwrap();

        let a2 = Square::new(1, 0).unwrap();
        let a8 = Square::new(7, 0).unwrap();

        let a1 = Square::new(0, 0).unwrap();
        let e1 = Square::new(0, 4).unwrap();
        let e3 = Square::new(2, 0).unwrap();
        let a3 = Square::new(2, 4).unwrap();

        let standard = MoveData::new_standard(d2, d5);
        let double_step = MoveData::new_double_step(e7, e5, e6);
        let en_passant = MoveData::new_en_passant(d5, e6, e5);
        let promotion = MoveData::new_promotion(a2, a8, Some(Title::Queen));
        let casteling = MoveData::new_casteling(e1, e3, a1, a3);
        let mut board = Board::new_standard();
        board.make_move(standard);
        assert!(!board.pieces.contains_key(&d2));
        assert_eq!(board.pieces[&d5], Piece::new(Color::White, Title::Pawn));
        assert_eq!(board.captures.len(), 0);
        board.make_move(double_step);
        assert_eq!(board.en_passant.unwrap(), e6);
        board.make_move(en_passant);
        assert_eq!(board.captures.len(), 1);
        board.make_move(promotion);
        assert!(!board.pieces.contains_key(&a2));
        assert_eq!(board.pieces[&a8], Piece::new(Color::White, Title::Queen));
        board.make_move(casteling);
        assert!(!board.pieces.contains_key(&a1));
        assert!(!board.pieces.contains_key(&e1));
        assert_eq!(board.pieces[&a3], Piece::new(Color::White, Title::Rook));
        assert_eq!(board.pieces[&e3], Piece::new(Color::White, Title::King));
    }
    #[test]
    fn king_pos() {
        let e1 = Square::new(0, 4).unwrap();
        let e8 = Square::new(7, 4).unwrap();
        let a4 = Square::new(3, 0).unwrap();

        let mut board = Board::new_standard();
        board.update_king_pos();

        assert_eq!(board.king_pos[0], e1);
        assert_eq!(board.king_pos[1], e8);

        let md = MoveData::new_standard(e1, a4);
        board.make_move(md);
        assert!(board.pieces.contains_key(&a4));
        board.update_king_pos();
        assert_eq!(board.king_pos[0], a4);
        //assert_eq!(board.king_pos[1], e8);
    }
    #[test]
    fn new_empty() {
        let new_empty = Board::new_empty();
        assert_eq!(new_empty.to_act, Color::White);
        assert_eq!(new_empty.captures.len(), 0);
        assert_eq!(new_empty.pieces.len(), 0);
    }
    #[test]
    fn new_standard() {
        let new_std = Board::new_standard();
        assert_eq!(new_std.to_act, Color::White);
        assert_eq!(new_std.captures.len(), 0);
        assert_eq!(new_std.pieces.len(), 32);
    }
    #[test]
    fn standard_content() {
        let new_std = Board::new_standard();
        assert_eq!(
            new_std.pieces[&Square::new(1, 0).unwrap()],
            Piece::new(Color::White, Title::Pawn)
        );
        assert_eq!(
            new_std.pieces[&Square::new(1, 4).unwrap()],
            Piece::new(Color::White, Title::Pawn)
        );
        assert_eq!(
            new_std.pieces[&Square::new(1, 7).unwrap()],
            Piece::new(Color::White, Title::Pawn)
        );
        assert_eq!(
            new_std.pieces[&Square::new(6, 6).unwrap()],
            Piece::new(Color::Black, Title::Pawn)
        );
        assert_eq!(
            new_std.pieces[&Square::new(6, 7).unwrap()],
            Piece::new(Color::Black, Title::Pawn)
        );
        assert_eq!(
            new_std.pieces[&Square::new(6, 5).unwrap()],
            Piece::new(Color::Black, Title::Pawn)
        );

        assert_eq!(
            new_std.pieces[&Square::new(0, 0).unwrap()],
            Piece::new(Color::White, Title::Rook)
        );
        assert_eq!(
            new_std.pieces[&Square::new(0, 1).unwrap()],
            Piece::new(Color::White, Title::Knight)
        );
        assert_eq!(
            new_std.pieces[&Square::new(0, 4).unwrap()],
            Piece::new(Color::White, Title::King)
        );
        assert_eq!(
            new_std.pieces[&Square::new(7, 3).unwrap()],
            Piece::new(Color::Black, Title::Queen)
        );
        assert_eq!(
            new_std.pieces[&Square::new(7, 2).unwrap()],
            Piece::new(Color::Black, Title::Bishop)
        );
        assert_eq!(
            new_std.pieces[&Square::new(7, 4).unwrap()],
            Piece::new(Color::Black, Title::King)
        );
    }

    #[test]
    fn occupancy() {
        let new_std = Board::new_standard();
        assert_eq!(
            new_std.occupancy(Square::new(4, 4).unwrap(), Color::Black),
            Occupancy::Empty
        );
        assert_eq!(
            new_std.occupancy(Square::new(4, 4).unwrap(), Color::White),
            Occupancy::Empty
        );
        assert_eq!(
            new_std.occupancy(Square::new(1, 0).unwrap(), Color::Black),
            Occupancy::OtherColor
        );
        assert_eq!(
            new_std.occupancy(Square::new(1, 0).unwrap(), Color::White),
            Occupancy::SameColor
        );
        assert_eq!(
            new_std.occupancy(Square::new(7, 1).unwrap(), Color::White),
            Occupancy::OtherColor
        );
        assert_eq!(
            new_std.occupancy(Square::new(7, 1).unwrap(), Color::Black),
            Occupancy::SameColor
        );
    }
    #[test]
    fn primitive_piece_manipluation() {
        let mut board = Board::new_empty();

        board.pieces.insert(
            Square::new(7, 7).unwrap(),
            Piece::new(Color::Black, Title::Queen),
        );
        assert_eq!(
            board.pieces[&Square::new(7, 7).unwrap()],
            Piece::new(Color::Black, Title::Queen)
        );
        assert_eq!(
            board
                .replace_piece(
                    Square::new(7, 7).unwrap(),
                    Piece::new(Color::White, Title::Bishop)
                )
                .unwrap(),
            Piece::new(Color::Black, Title::Queen)
        );
        board.replace_piece_with_capture(
            Square::new(7, 7).unwrap(),
            Piece::new(Color::Black, Title::Rook),
        );
        assert_eq!(board.captures[0], Piece::new(Color::White, Title::Bishop));
        board.remove_piece(Square::new(7, 7).unwrap());
        assert!(!board.pieces.contains_key(&Square::new(7, 7).unwrap()));
    }
    #[test]
    fn move_piece_manipluation() {
        let mut board = Board::new_empty();
        let (a, b, c) = (
            Square::new(7, 7).unwrap(),
            Square::new(5, 5).unwrap(),
            Square::new(4, 4).unwrap(),
        );
        board
            .pieces
            .insert(a, Piece::new(Color::White, Title::Queen));
        board
            .pieces
            .insert(b, Piece::new(Color::Black, Title::Knight));
        board
            .pieces
            .insert(c, Piece::new(Color::Black, Title::Bishop));
        board.move_piece(a, b);
        assert_eq!(board.captures.len(), 0);
        assert!(!board.pieces.contains_key(&a));
        assert_eq!(board.pieces[&b], Piece::new(Color::White, Title::Queen));
        board.move_piece_with_capture(b, c);
        assert_eq!(board.captures.len(), 1);
        assert!(!board.pieces.contains_key(&b));
        assert_eq!(board.pieces[&c], Piece::new(Color::White, Title::Queen));
    }
}
