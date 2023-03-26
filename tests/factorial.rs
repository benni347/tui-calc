use tuicalc::factorial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        /*
        assert_eq!(
            factorial(-2.0),
            Err("Values smaller 0 aren't implemented yet".to_string())
        );
        assert_eq!(
            factorial(-10.0),
            Err("Values smaller 0 aren't implemented yet".to_string())
        );
        assert_eq!(
            factorial(-15.0),
            Err("Values smaller 0 aren't implemented yet".to_string())
        );
        */
    }
}
