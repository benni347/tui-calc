use tuicalc::add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(&[0, 0]), 0);
        assert_eq!(add(&[0, 5]), 5);
        assert_eq!(add(&[5, 7]), 12);
        assert_eq!(add(&[0, 1]), 1);
        assert_eq!(add(&[123, 456]), 579);
        assert_eq!(add(&[-123, 456]), 333);
        assert_eq!(add(&[123, -456]), -333);
        assert_eq!(add(&[-123, -456]), -579);
        assert_eq!(
            add(&[123456789123456789, 987654321987654321],),
            1111111111111111110,
        );
    }
}
