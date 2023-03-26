use crate::pi;

pub fn circle(r: f64) -> f64 {
    // This function calculates the area of a circle
    pi() * r.powi(2)
}

pub fn circle_circumference(r: f64) -> f64 {
    // This function calculates the circumference of a circle
    2.0 * pi() * r
}
