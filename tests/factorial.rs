use tuicalc::factorials;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorials::factorial(0), 1);
        assert_eq!(factorials::factorial(1), 1);
        assert_eq!(factorials::factorial(2), 2);
        assert_eq!(factorials::factorial(5), 120);
        assert_eq!(factorials::factorial(10), 3628800);
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

    #[test]
    fn test_hyper_factorial() {
        assert_eq!(factorials::hyperfactorial(1), 1);
        assert_eq!(factorials::hyperfactorial(2), 4);
        assert_eq!(factorials::hyperfactorial(5), 86400000);
    }

    #[test]
    fn test_small_multifactorial() {
        assert_eq!(factorials::multifactorial(1, 2), 1);
        assert_eq!(factorials::multifactorial(2, 3), 2);
        assert_eq!(factorials::multifactorial(5, 10), 5);
        assert_eq!(factorials::multifactorial(15, 3), 29160);
    }

    #[test]
    fn test_big_multifactorial() {
        assert_eq!(factorials::multifactorial(1, 100), 1);
        assert_eq!(factorials::multifactorial(2, 100), 2);
        assert_eq!(factorials::multifactorial(5, 100), 5);
        assert_eq!(factorials::multifactorial(15, 100), 15);
    }
}
