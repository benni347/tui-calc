use crate::circle::circle;
use crate::pi;
use crate::triangle::pythagorean_theorem;

pub fn cone(r: f64, h: f64) -> f64 {
    // This function calculates the volume of a cone
    // https://mathworld.wolfram.com/Cone.html
    circle(r) * h / 3.0
}

pub fn cone_surface_area(r: f64, h: f64) -> f64 {
    // This function calculates the surface area of a cone
    // https://mathworld.wolfram.com/Cone.html
    circle(r) + pi() * r * pythagorean_theorem(r, h)
}
