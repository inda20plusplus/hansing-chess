#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Space(i32, i32);

impl Space {
    pub fn is_in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }
    pub fn offset(&self, rank_offset: i32, file_offset: i32) -> Self {
        Self(self.0 + rank_offset, self.1 + file_offset)
    }
    pub fn new(rank: i32, file: i32)->Self{
        Self(rank, file)
    }
}

#[cfg(test)]
mod space{
    use super::*;
    #[test]
    fn out_of_bounds_space() {
        assert!(Space(-1, 3).is_in_bounds());
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
    fn constructor(){
        let s = Space::new(2,3);
        assert_eq!(s, Space(2,3))
    }
}