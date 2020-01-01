fn main(x: i32) -> i32 {
    x * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeed_test() {
        assert_eq!(8, main(2));
    }

    #[test]
    fn failure_test() {
        assert_eq!(4, main(2));
    }
}