#[derive(PartialEq, Debug)]
pub enum Occupancy {
    Empty,
    SameColor,
    OtherColor,
}

#[cfg(test)]
mod test_occupancy {
    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Occupancy::OtherColor, Occupancy::OtherColor)
    }
}
