use tuicalc::multiply;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&[0, 0]), 0);
        assert_eq!(multiply(&[0, 5]), 0);
        assert_eq!(multiply(&[5, 0]), 0);
        assert_eq!(multiply(&[2, 6]), 12);
        assert_eq!(multiply(&[1, 1]), 1);
        assert_eq!(multiply(&[123, 456]), 56088);
        assert_eq!(multiply(&[-123, 456]), -56088);
        assert_eq!(multiply(&[123, -456]), -56088);
        assert_eq!(multiply(&[-123, -456]), 56088);
        assert_eq!(
            multiply(&[123456789123456789, 987654321987654321],),
            121932631356500531347203169112635269,
        );
    }
}
