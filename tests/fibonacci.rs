use tuicalc::fibonacci::fibonacci;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fionacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(11), 89);
        assert_eq!(fibonacci(12), 144);
        assert_eq!(fibonacci(13), 233);
        assert_eq!(fibonacci(14), 377);
        assert_eq!(fibonacci(15), 610);
        assert_eq!(fibonacci(16), 987);
        assert_eq!(fibonacci(17), 1597);
        assert_eq!(fibonacci(18), 2584);
        assert_eq!(fibonacci(19), 4181);
        assert_eq!(fibonacci(20), 6765);
        assert_eq!(fibonacci(21), 10946);
        assert_eq!(fibonacci(22), 17711);
        assert_eq!(fibonacci(23), 28657);
        assert_eq!(fibonacci(24), 46368);
        assert_eq!(fibonacci(25), 75025);
        assert_eq!(fibonacci(26), 121393);
        assert_eq!(fibonacci(27), 196418);
        assert_eq!(fibonacci(28), 317811);
        assert_eq!(fibonacci(29), 514229);
        assert_eq!(fibonacci(30), 832040);
        assert_eq!(fibonacci(31), 1346269);
        assert_eq!(fibonacci(32), 2178309);
        assert_eq!(fibonacci(33), 3524578);
        assert_eq!(fibonacci(34), 5702887);
        assert_eq!(fibonacci(35), 9227465);
        assert_eq!(fibonacci(36), 14930352);
        assert_eq!(fibonacci(37), 24157817);
        assert_eq!(fibonacci(38), 39088169);
    }
}
