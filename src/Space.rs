

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