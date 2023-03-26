use tuicalc::triangle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pythagorean_therome() {
        assert_eq!(triangle::pythagorean_therome(3.0, 4.0), 5.0);
        assert_eq!(triangle::pythagorean_therome(5.0, 12.0), 13.0);
        assert_eq!(triangle::pythagorean_therome(8.0, 15.0), 17.0);
        assert_eq!(triangle::pythagorean_therome(7.0, 24.0), 25.0);
        assert_eq!(triangle::pythagorean_therome(20.0, 21.0), 29.0);
    }

    #[test]
    fn pythagorean_therome_c() {
        assert_eq!(triangle::pythagorean_therome_c(3.0, 5.0), 4.0);
        assert_eq!(triangle::pythagorean_therome_c(5.0, 13.0), 12.0);
        assert_eq!(triangle::pythagorean_therome_c(8.0, 17.0), 15.0);
        assert_eq!(triangle::pythagorean_therome_c(7.0, 25.0), 24.0);
        assert_eq!(triangle::pythagorean_therome_c(20.0, 29.0), 21.0);
    }
}
