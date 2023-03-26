use crate::circle::circle;
use crate::pi;

pub fn cone(r: f64, h: f64) -> f64 {
    // This function calculates the volume of a cone
    // https://mathworld.wolfram.com/Cone.html
    circle(r) * h / 3.0
}

pub fn cone_surface_area(r: f64, h: f64) -> f64 {
    // This function calculates the surface area of a cone
    // https://mathworld.wolfram.com/Cone.html
    // TODO: As soon as I have the pythagorean theorem, I can use it here
    // instead of the sqrt function
    circle(r) + pi() * r * (r.powi(2) + h.powi(2)).sqrt()
}
