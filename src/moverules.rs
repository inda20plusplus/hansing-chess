pub const KNIGHT_MOVES: [(i32, i32, bool); 8] = [
    (2, 1, false),
    (2, -1, false),
    (-2, 1, false),
    (-2, -1, false),
    (1, 2, false),
    (-1, 2, false),
    (1, -2, false),
    (-1, -2, false),
];
pub const BISHOP_MOVES: [(i32, i32, bool); 4] =
    [(1, 1, true), (-1, -1, true), (-1, 1, true), (1, -1, true)];
pub const ROOK_MOVES: [(i32, i32, bool); 4] =
    [(1, 0, true), (-1, 0, true), (0, 1, true), (0, -1, true)];
pub const QUEEN_MOVES: [(i32, i32, bool); 8] = [
    (1, 0, true),
    (-1, 0, true),
    (0, 1, true),
    (0, -1, true),
    (1, 1, true),
    (-1, -1, true),
    (-1, 1, true),
    (1, -1, true),
];
pub const KING_MOVES: [(i32, i32, bool); 8] = [
    (1, 0, false),
    (-1, 0, false),
    (0, 1, false),
    (0, -1, false),
    (1, 1, false),
    (-1, -1, false),
    (-1, 1, false),
    (1, -1, false),
];
