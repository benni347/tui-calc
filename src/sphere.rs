use crate::pi;

pub fn sphere(r: f64) -> f64 {
    // This function calculates the volume of a sphere
    4.0 / 3.0 * pi() * r.powi(3)
}

pub fn sphere_surface(r: f64) -> f64 {
    // This function calculates the surface area of a sphere
    4.0 * pi() * r.powi(2)
}
