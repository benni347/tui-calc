use tuicalc::rectangle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(rectangle::rectangle(0.0, 0.0), 0.0);
        assert_eq!(rectangle::rectangle(1.0, 1.0), 1.0);
        assert_eq!(rectangle::rectangle(2.0, 2.0), 4.0);
        assert_eq!(rectangle::rectangle(5.0, 5.0), 25.0);
        assert_eq!(rectangle::rectangle(10.0, 10.0), 100.0);
        assert_eq!(rectangle::rectangle(-2.0, -2.0), 4.0);
        assert_eq!(rectangle::rectangle(-10.0, -10.0), 100.0);
        assert_eq!(rectangle::rectangle(-15.0, -15.0), 225.0);
    }

    #[test]
    fn test_perimeter() {
        assert_eq!(rectangle::rectangle_perimeter(0.0, 0.0), 0.0);
        assert_eq!(rectangle::rectangle_perimeter(1.0, 1.0), 4.0);
        assert_eq!(rectangle::rectangle_perimeter(2.0, 2.0), 8.0);
        assert_eq!(rectangle::rectangle_perimeter(5.0, 5.0), 20.0);
        assert_eq!(rectangle::rectangle_perimeter(10.0, 10.0), 40.0);
        assert_eq!(rectangle::rectangle_perimeter(-2.0, -2.0), -8.0);
        assert_eq!(rectangle::rectangle_perimeter(-10.0, -10.0), -40.0);
        assert_eq!(rectangle::rectangle_perimeter(-15.0, -15.0), -60.0);
    }
}
