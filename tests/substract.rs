use tuicalc::substract;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substract() {
        assert_eq!(substract(&[0, 0]), 0);
        assert_eq!(substract(&[0, 5]), -5);
        assert_eq!(substract(&[5, 0]), 5);
        assert_eq!(substract(&[1, 1]), 0);
        assert_eq!(substract(&[123, 456]), -333);
        assert_eq!(substract(&[-123, 456]), -579);
        assert_eq!(substract(&[123, -456]), 579);
        assert_eq!(substract(&[-123, -456]), 333);
        assert_eq!(
            substract(&[123456789123456789, 987654321987654321],),
            -864197532864197532,
        );
    }
}
