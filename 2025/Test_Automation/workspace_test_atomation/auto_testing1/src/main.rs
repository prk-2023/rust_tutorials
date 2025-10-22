pub fn area_of_rectangle(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("area of rectangle : {}", area_of_rectangle(10, 20))
}

#[cfg(test)]
mod testing {
    use super::*; // Brings the `area_of_rectangle` function into scope

    #[test]
    fn test_area_of_rectangle() {
        assert_eq!(area_of_rectangle(3, 4), 12); // 3 * 4 = 12
        assert_eq!(area_of_rectangle(5, 6), 30); // 5 * 6 = 30
    }
    #[test]
    fn reverse_test_area_of_rectangle() {
        assert_ne!(area_of_rectangle(3, 4), 14); // 3 * 4 = 12
        assert_ne!(area_of_rectangle(5, 6), 32); // 5 * 6 = 30
    }
}
