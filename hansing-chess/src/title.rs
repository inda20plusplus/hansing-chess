#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Title {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[cfg(test)]
mod test_title {
    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Title::Bishop, Title::Bishop)
    }
}
