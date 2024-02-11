pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        println!(ten_is_bigger_than_eight())
    }else {
        println!(b)
    }
}

//println!()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}