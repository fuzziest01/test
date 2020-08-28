fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let expected = 6;
        let received = add(2, 4);

        assert_eq!(expected, received);
    }
}
