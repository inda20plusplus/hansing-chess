use crate::square::Square;
use crate::title::Title;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MoveData {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}

impl MoveData {
    pub fn new_standard(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Standard,
        }
    }
    pub fn new_casteling(from: Square, to: Square, rook_from: Square, rook_to: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Casteling(rook_from, rook_to),
        }
    }
    pub fn new_en_passant(from: Square, to: Square, capture: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::EnPassant(capture),
        }
    }
    pub fn new_double_step(from: Square, to: Square, en_passant_oppertunity: Square) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::DoubleStep(en_passant_oppertunity),
        }
    }
    pub fn new_promotion(from: Square, to: Square, promote_to: Option<Title>) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Promotion(promote_to),
        }
    }

    pub fn get_move_notation(&self) -> String {
        format!("{} {}", self.from, self.to)
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveType {
    Standard,
    Casteling(Square, Square), //Rook from, Rook to
    EnPassant(Square),         //Capture
    DoubleStep(Square),        //En passant opertunity
    Promotion(Option<Title>),  //Promote to
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::title::Title;
    #[test]
    fn new_movedata() {
        let (from, to, capture, rook_from, rook_to, en_passant_oppertunity) = (
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
        );
        let promote_to = Some(Title::Bishop);

        let _standard = MoveData::new_standard(from, to);
        let _en_passant = MoveData::new_en_passant(from, to, capture);
        let _promotion = MoveData::new_promotion(from, to, promote_to);
        let _double_step = MoveData::new_double_step(from, to, en_passant_oppertunity);
        let _casteling = MoveData::new_casteling(from, to, rook_from, rook_to);
    }

    #[test]
    fn eq() {
        let (from, to, _capture, rook_from, rook_to) = (
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
            Square::new(3, 4).unwrap(),
        );
        let standard = MoveData::new_standard(from, to);
        let casteling = MoveData::new_casteling(from, to, rook_from, rook_to);
        assert_eq!(standard.from, Square::new(3, 4).unwrap());
        assert_eq!(
            casteling.move_type,
            MoveType::Casteling(Square::new(3, 4).unwrap(), Square::new(3, 4).unwrap())
        );
    }
}
