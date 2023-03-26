pub fn pythagorean_theorem(a: f64, b: f64) -> f64 {
    // This function calculates the hypotenuse of a right triangle
    // https://mathworld.wolfram.com/PythagoreanTheorem.html
    (a.powi(2) + b.powi(2)).sqrt()
}

pub fn pythagorean_theorem_c(a: f64, c: f64) -> f64 {
    // This function calculates the hypotenuse of a right triangle
    // https://mathworld.wolfram.com/PythagoreanTheorem.html
    (c.powi(2) - a.powi(2)).sqrt()
}
