use tuicalc::binomial_coefficient::binomial_coefficient;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(binomial_coefficient(0, 0), 1);
        assert_eq!(binomial_coefficient(1, 0), 1);
        assert_eq!(binomial_coefficient(1, 1), 1);
        assert_eq!(binomial_coefficient(2, 0), 1);
        assert_eq!(binomial_coefficient(2, 1), 2);
        assert_eq!(binomial_coefficient(2, 2), 1);
        assert_eq!(binomial_coefficient(3, 0), 1);
        assert_eq!(binomial_coefficient(3, 1), 3);
        assert_eq!(binomial_coefficient(3, 2), 3);
        assert_eq!(binomial_coefficient(3, 3), 1);
        assert_eq!(binomial_coefficient(4, 0), 1);
        assert_eq!(binomial_coefficient(4, 1), 4);
        assert_eq!(binomial_coefficient(4, 2), 6);
        assert_eq!(binomial_coefficient(4, 3), 4);
        assert_eq!(binomial_coefficient(4, 4), 1);
        assert_eq!(binomial_coefficient(5, 0), 1);
        assert_eq!(binomial_coefficient(5, 1), 5);
        assert_eq!(binomial_coefficient(5, 2), 10);
        assert_eq!(binomial_coefficient(5, 3), 10);
        assert_eq!(binomial_coefficient(5, 4), 5);
        assert_eq!(binomial_coefficient(5, 5), 1);
        assert_eq!(binomial_coefficient(6, 0), 1);
        assert_eq!(binomial_coefficient(6, 1), 6);
        assert_eq!(binomial_coefficient(6, 2), 15);
        assert_eq!(binomial_coefficient(6, 3), 20);
        assert_eq!(binomial_coefficient(6, 4), 15);
        assert_eq!(binomial_coefficient(6, 5), 6);
        assert_eq!(binomial_coefficient(6, 6), 1);
        assert_eq!(binomial_coefficient(7, 0), 1);
    }
}
