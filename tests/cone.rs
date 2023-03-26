use tuicalc::cone;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        assert_eq!(cone::cone_surface_area(1.0, 1.0), 7.584475591748159);
        assert_eq!(cone::cone_surface_area(2.0, 2.0), 30.337902366992637);
        assert_eq!(cone::cone_surface_area(3.0, 3.0), 68.26028032573343);
        assert_eq!(cone::cone_surface_area(4.0, 4.0), 121.35160946797055);
        assert_eq!(cone::cone_surface_area(5.0, 5.0), 189.611889793704);
    }

    #[test]
    fn volume() {
        assert_eq!(cone::cone(1.0, 1.0), 1.0471975511965976);
        assert_eq!(cone::cone(2.0, 2.0), 8.377580409572781);
        assert_eq!(cone::cone(3.0, 3.0), 28.274333882308138);
        assert_eq!(cone::cone(4.0, 4.0), 67.02064327658225);
        assert_eq!(cone::cone(5.0, 5.0), 130.89969389957471);
    }
}
