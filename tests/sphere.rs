use tuicalc::sphere;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_volume() {
        assert_eq!(sphere::sphere(0.0), 0.0);
        assert_eq!(sphere::sphere(1.0), 4.1887902047863905);
        assert_eq!(sphere::sphere(2.0), 33.510321638291124);
        assert_eq!(sphere::sphere(5.0), 523.5987755982989);
        assert_eq!(sphere::sphere(10.0), 4188.7902047863905);
    }

    #[test]
    fn test_sphere_surface() {
        assert_eq!(sphere::sphere_surface(0.0), 0.0);
        assert_eq!(sphere::sphere_surface(1.0), 12.566370614359172);
        assert_eq!(sphere::sphere_surface(2.0), 50.26548245743669);
        assert_eq!(sphere::sphere_surface(5.0), 314.1592653589793);
        assert_eq!(sphere::sphere_surface(10.0), 1256.6370614359172);
    }
}
