pub mod circle;
pub mod factorials;
pub mod rectangle;
pub mod sphere;
pub mod square;

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
