use crate::{phi, psi};

pub fn fibonacci(n: i128) -> i128 {
    // This function calculates the nth fibonacci number
    // https://mathworld.wolfram.com/FibonacciNumber.html
    ((phi().powi(n as i32) - psi().powi(n as i32)) / 5f64.sqrt()).round() as i128
}
