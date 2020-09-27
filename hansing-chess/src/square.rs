use std::fmt;
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Square(i32, i32);

impl Square {
    pub fn is_in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }
    pub fn offset(&self, rank_offset: i32, file_offset: i32) -> Option<Self> {
        let s = Self(self.0 + rank_offset, self.1 + file_offset);
        if s.is_in_bounds() {
            Some(s)
        } else {
            None
        }
    }
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let s = Square(rank, file);
        if s.is_in_bounds() {
            Some(s)
        } else {
            None
        }
    }
    pub fn rank(&self) -> i32 {
        self.0
    }
    pub fn file(&self) -> i32 {
        self.1
    }
}

pub const FILE_SIGN: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", FILE_SIGN[self.1 as usize], self.0 + 1)
    }
}

#[cfg(test)]
mod test_square {
    use super::*;
    #[test]
    fn eq() {
        let s1 = Square::new(1, 4).unwrap();
        let s2 = s1.clone();
        assert_eq!(s1, s2)
    }
    #[test]
    fn out_of_bounds() {
        let s1 = Square::new(-1, 4);
        let s2 = Square::new(1, 8);
        let s3 = Square::new(6, -1);
        let s4 = Square::new(8, 4);
        assert_eq!(s1, None);
        assert_eq!(s2, None);
        assert_eq!(s3, None);
        assert_eq!(s4, None);
    }
    #[test]
    fn offset() {
        let s = Square::new(4, 4).unwrap();
        let s1 = s.offset(2, 2);
        let s2 = s.offset(7, -7);
        assert_eq!(s1, Some(Square::new(6, 6).unwrap()));
        assert_eq!(s2, None);
    }
}
