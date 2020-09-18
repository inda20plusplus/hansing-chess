use crate::board::Board;
use crate::movedata::MoveData;
use crate::occupancy::Occupancy;
use crate::square::Square;
use crate::title::Title;

use crate::moverules::*;

pub static mut PROMOTE_TO: Title = Title::Queen;

pub fn generate_action_space(board: Board) -> Vec<MoveData> {
    let mut action_space: Vec<MoveData> = Vec::new();

    for from in board.pieces.keys() {
        if board.pieces[&from].color == board.to_act {
            if board.pieces[&from].title == Title::Pawn {
                action_space.append(&mut pawn_moves_from(&board, *from));
            } else {
                action_space.append(&mut piece_moves_from(&board, *from));
            }
        }
    }
    action_space
}

fn pawn_moves_from(board: &Board, from: Square) -> Vec<MoveData> {
    let mut moves: Vec<MoveData> = Vec::new();
    let piece = board.pieces[&from];
    //forward
    if let Some(to_step) = from.offset(piece.color.forward(), 0) {
        if board.occupancy(to_step, piece.color) == Occupancy::Empty {
            if from.rank() == piece.color.seventh_rank() {
                unsafe {
                    //oof
                    add_move(
                        MoveData::new_promotion(from, to_step, Some(PROMOTE_TO)),
                        &mut moves,
                        board,
                    );
                }
            } else {
                add_move(MoveData::new_standard(from, to_step), &mut moves, board);
                if !piece.has_moved && board.occupancy(to_step, piece.color) == Occupancy::Empty {
                    if let Some(to_double_step) = to_step.offset(piece.color.forward(), 0) {
                        add_move(
                            MoveData::new_double_step(from, to_double_step, to_step),
                            &mut moves,
                            board,
                        );
                    }
                }
            }
        }
    }
    //Capture
    for &file_offset in [1, -1].iter() {
        if let Some(to_capture) = from.offset(piece.color.forward(), file_offset) {
            if board.occupancy(to_capture, piece.color) == Occupancy::OtherColor {
                if from.rank() == piece.color.seventh_rank() {
                    unsafe {
                        //oof
                        add_move(
                            MoveData::new_promotion(from, to_capture, Some(PROMOTE_TO)),
                            &mut moves,
                            board,
                        );
                    }
                } else {
                    add_move(MoveData::new_standard(from, to_capture), &mut moves, board);
                }
            } else if board.en_passant == Some(to_capture) {
                moves.push(MoveData::new_en_passant(
                    from,
                    to_capture,
                    to_capture
                        .offset(piece.color.inverse().forward(), 0)
                        .unwrap(),
                ))
            }
        }
    }
    moves
}

fn piece_moves_from(board: &Board, from: Square) -> Vec<MoveData> {
    let piece = board.pieces[&from];
    let mut moves: Vec<MoveData> = Vec::new();
    let move_directions = match piece.title {
        Title::Knight => KNIGHT_MOVES.to_vec(),
        Title::Rook => ROOK_MOVES.to_vec(),
        Title::Bishop => BISHOP_MOVES.to_vec(),
        Title::Queen => QUEEN_MOVES.to_vec(),
        Title::King => KING_MOVES.to_vec(),
        _ => [].to_vec(),
    };
    for (rank_move, file_move, repeating) in move_directions {
        let mut to = from;
        loop {
            if let Some(temp) = to.offset(rank_move, file_move) {
                to = temp;
                if board.occupancy(to, piece.color) == Occupancy::Empty {
                    add_move(MoveData::new_standard(from, to), &mut moves, board);

                    if !repeating {
                        break;
                    }
                } else if board.occupancy(to, piece.color) == Occupancy::OtherColor {
                    add_move(MoveData::new_standard(from, to), &mut moves, board);

                    break;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    //Casteling moves
    if piece.title == Title::King && !piece.has_moved && !board.in_check {
        // King side
        if board.occupancy(from.offset(0, 1).unwrap(), piece.color) == Occupancy::Empty
            && board.occupancy(from.offset(0, 2).unwrap(), piece.color) == Occupancy::Empty
            && board.occupancy(from.offset(0, 3).unwrap(), piece.color) == Occupancy::SameColor
        {
            if !board.pieces[&from.offset(0, 3).unwrap()].has_moved {
                let m = MoveData::new_casteling(
                    from,
                    from.offset(0, 2).unwrap(),
                    from.offset(0, 3).unwrap(),
                    from.offset(0, 1).unwrap(),
                );
                let mut result = board.clone();
                result.make_move(m);

                let (c1, _) = result.check_if_threatend(from, board.to_act);
                let (c2, _) = result.check_if_threatend(from.offset(0, 1).unwrap(), board.to_act);
                let (c3, _) = result.check_if_threatend(from.offset(0, 2).unwrap(), board.to_act);

                if !c1 && !c2 && !c3 {
                    moves.push(m)
                }
            }
        }
        // Queen side
        if board.occupancy(from.offset(0, -1).unwrap(), piece.color) == Occupancy::Empty
            && board.occupancy(from.offset(0, -2).unwrap(), piece.color) == Occupancy::Empty
            && board.occupancy(from.offset(0, -3).unwrap(), piece.color) == Occupancy::Empty
            && board.occupancy(from.offset(0, -4).unwrap(), piece.color) == Occupancy::SameColor
        {
            if !board.pieces[&from.offset(0, -4).unwrap()].has_moved {
                let m = MoveData::new_casteling(
                    from,
                    from.offset(0, -3).unwrap(),
                    from.offset(0, -4).unwrap(),
                    from.offset(0, -2).unwrap(),
                );
                let mut result = board.clone();
                result.make_move(m);

                let (c1, _) = result.check_if_threatend(from, board.to_act);
                let (c2, _) = result.check_if_threatend(from.offset(0, -1).unwrap(), board.to_act);
                let (c3, _) = result.check_if_threatend(from.offset(0, -2).unwrap(), board.to_act);
                let (c4, _) = result.check_if_threatend(from.offset(0, -3).unwrap(), board.to_act);

                if !c1 && !c2 && !c3 && !c4 {
                    moves.push(m)
                }
            }
        }
    }

    moves
}

fn add_move(m: MoveData, buffer: &mut Vec<MoveData>, board: &Board) {
    if board.blockers.contains(&m.from) || board.in_check {
        let mut result = board.clone();
        result.make_move(m);
        let (c, _) = result.check_if_threatend(result.king_pos[board.to_act.index()], board.to_act);
        if c {
            return;
        }
    }
    buffer.push(m)
}
