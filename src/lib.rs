pub mod binomial_coefficient;
pub mod circle;
pub mod cone;
pub mod factorials;
pub mod fibonacci;
pub mod rectangle;
pub mod sphere;
pub mod square;
pub mod triangle;

pub fn add(nums: &[i128]) -> i128 {
    nums.iter().sum()
}

pub fn multiply(nums: &[i128]) -> i128 {
    nums.iter().product()
}

pub fn substract(nums: &[i128]) -> i128 {
    let mut result = nums[0];
    for num in &nums[1..] {
        result -= *num;
    }
    result
}

pub fn pi() -> f64 {
    std::f64::consts::PI
}

pub fn phi() -> f64 {
    (1.0 + 5f64.sqrt()) / 2.0
}

pub fn psi() -> f64 {
    (1.0 - 5f64.sqrt()) / 2.0
}
