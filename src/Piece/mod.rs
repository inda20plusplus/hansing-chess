use super::Color::Color;
mod MoveRules;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self {
            color,
            piece_type,
            has_moved: false,
        }
    }

    pub fn get_move_pattern(&self) -> Vec<(i32, i32, bool)> {
        match self.piece_type {
            //PieceType::Pawn =>
            PieceType::Knight => MoveRules::KNIGHT_MOVES.to_vec(),

            PieceType::Bishop => MoveRules::BISHOP_MOVES.to_vec(),

            PieceType::Rook => MoveRules::ROOK_MOVES.to_vec(),

            PieceType::Queen => MoveRules::QUEEN_MOVES.to_vec(),

            PieceType::King => MoveRules::KING_MOVES.to_vec(),

            _ => Vec::new(),
        }
    }
    /*
    pub fn get_move_pattern(&self) -> Vec<(i32, i32, bool)> {
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

            PieceType::Rook => vec![(1, 0, true), (-1, 0, true), (0, 1, true), (0, -1, true)],

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
            ],
        }
    }*/
}
#[cfg(test)]
mod piece {
    use super::*;
    #[test]
    fn piece_type_equals() {
        assert_eq!(PieceType::Bishop, PieceType::Bishop);
        assert_eq!(PieceType::King, PieceType::King);
        assert_ne!(PieceType::Rook, PieceType::Pawn);
    }
}
