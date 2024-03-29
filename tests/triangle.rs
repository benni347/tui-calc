use tuicalc::triangle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pythagorean_therome() {
        assert_eq!(triangle::pythagorean_theorem(3.0, 4.0), 5.0);
        assert_eq!(triangle::pythagorean_theorem(5.0, 12.0), 13.0);
        assert_eq!(triangle::pythagorean_theorem(8.0, 15.0), 17.0);
        assert_eq!(triangle::pythagorean_theorem(7.0, 24.0), 25.0);
        assert_eq!(triangle::pythagorean_theorem(20.0, 21.0), 29.0);
    }

    #[test]
    fn pythagorean_therome_c() {
        assert_eq!(triangle::pythagorean_theorem_c(3.0, 5.0), 4.0);
        assert_eq!(triangle::pythagorean_theorem_c(5.0, 13.0), 12.0);
        assert_eq!(triangle::pythagorean_theorem_c(8.0, 17.0), 15.0);
        assert_eq!(triangle::pythagorean_theorem_c(7.0, 25.0), 24.0);
        assert_eq!(triangle::pythagorean_theorem_c(20.0, 29.0), 21.0);
    }
}
