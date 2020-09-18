use crate::color::Color;
use crate::piece::Piece;
use crate::square::Square;
use crate::title::Title;
use std::collections::HashMap;

pub fn standard_setup(pieces: &mut HashMap<Square, Piece>) {
    pieces.clear();
    pieces.insert(
        Square::new(0, 0).unwrap(),
        Piece::new(Color::White, Title::Rook),
    );
    pieces.insert(
        Square::new(0, 1).unwrap(),
        Piece::new(Color::White, Title::Knight),
    );
    pieces.insert(
        Square::new(0, 2).unwrap(),
        Piece::new(Color::White, Title::Bishop),
    );
    pieces.insert(
        Square::new(0, 3).unwrap(),
        Piece::new(Color::White, Title::Queen),
    );
    pieces.insert(
        Square::new(0, 4).unwrap(),
        Piece::new(Color::White, Title::King),
    );
    pieces.insert(
        Square::new(0, 5).unwrap(),
        Piece::new(Color::White, Title::Bishop),
    );
    pieces.insert(
        Square::new(0, 6).unwrap(),
        Piece::new(Color::White, Title::Knight),
    );
    pieces.insert(
        Square::new(0, 7).unwrap(),
        Piece::new(Color::White, Title::Rook),
    );
    // Secound rank (1): white frontline
    pieces.insert(
        Square::new(1, 0).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 1).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 2).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 3).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 4).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 5).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 6).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    pieces.insert(
        Square::new(1, 7).unwrap(),
        Piece::new(Color::White, Title::Pawn),
    );
    // Eight rank (7): black backline
    pieces.insert(
        Square::new(7, 0).unwrap(),
        Piece::new(Color::Black, Title::Rook),
    );
    pieces.insert(
        Square::new(7, 1).unwrap(),
        Piece::new(Color::Black, Title::Knight),
    );
    pieces.insert(
        Square::new(7, 2).unwrap(),
        Piece::new(Color::Black, Title::Bishop),
    );
    pieces.insert(
        Square::new(7, 3).unwrap(),
        Piece::new(Color::Black, Title::Queen),
    );
    pieces.insert(
        Square::new(7, 4).unwrap(),
        Piece::new(Color::Black, Title::King),
    );
    pieces.insert(
        Square::new(7, 5).unwrap(),
        Piece::new(Color::Black, Title::Bishop),
    );
    pieces.insert(
        Square::new(7, 6).unwrap(),
        Piece::new(Color::Black, Title::Knight),
    );
    pieces.insert(
        Square::new(7, 7).unwrap(),
        Piece::new(Color::Black, Title::Rook),
    );
    // Seventh rank (6): black frontline
    pieces.insert(
        Square::new(6, 0).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 1).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 2).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 3).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 4).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 5).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 6).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
    pieces.insert(
        Square::new(6, 7).unwrap(),
        Piece::new(Color::Black, Title::Pawn),
    );
}
