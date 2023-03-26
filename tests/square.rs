use tuicalc::square;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square::square(0.0), 0.0);
        assert_eq!(square::square(1.0), 1.0);
        assert_eq!(square::square(2.0), 4.0);
        assert_eq!(square::square(5.0), 25.0);
        assert_eq!(square::square(10.0), 100.0);
        assert_eq!(square::square(-2.0), 4.0);
        assert_eq!(square::square(-10.0), 100.0);
        assert_eq!(square::square(-15.0), 225.0);
    }

    #[test]
    fn test_square_perimeter() {
        assert_eq!(square::square_perimeter(0.0), 0.0);
        assert_eq!(square::square_perimeter(1.0), 4.0);
        assert_eq!(square::square_perimeter(2.0), 8.0);
        assert_eq!(square::square_perimeter(5.0), 20.0);
        assert_eq!(square::square_perimeter(10.0), 40.0);
        assert_eq!(square::square_perimeter(-2.0), -8.0);
        assert_eq!(square::square_perimeter(-10.0), -40.0);
        assert_eq!(square::square_perimeter(-15.0), -60.0);
    }
}
