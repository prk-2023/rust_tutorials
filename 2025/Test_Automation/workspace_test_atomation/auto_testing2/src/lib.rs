pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration3() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration4() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration5() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[should_panic(expected = "function call")]
    fn msg() {
        panic!("Making this test fail!");
    }
}
