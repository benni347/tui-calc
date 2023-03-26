use tuicalc::circle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        assert_eq!(circle::circle(0.0), 0.0);
        assert_eq!(circle::circle(1.0), 3.141592653589793);
        assert_eq!(circle::circle(2.0), 12.566370614359172);
        assert_eq!(circle::circle(5.0), 78.53981633974483);
        assert_eq!(circle::circle(10.0), 314.1592653589793);
    }

    #[test]
    fn test_circle_circumference() {
        assert_eq!(circle::circle_circumference(0.0), 0.0);
        assert_eq!(circle::circle_circumference(1.0), 6.283185307179586);
        assert_eq!(circle::circle_circumference(2.0), 12.566370614359172);
        assert_eq!(circle::circle_circumference(5.0), 31.41592653589793);
        assert_eq!(circle::circle_circumference(10.0), 62.83185307179586);
    }
}
