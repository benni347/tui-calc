pub mod circle;
pub mod sphere;

pub fn factorial(number: u128) -> u128 {
    let mut result: u128 = 1;
    for i in 1..=number {
        result *= i;
    }
    result
}

pub fn add(nums: &[i128]) -> i128 {
    nums.iter().fold(0, |acc, x| acc + x)
}

pub fn multiply(nums: &[i128]) -> i128 {
    nums.iter().fold(1, |acc, x| acc * x)
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
