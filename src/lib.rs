pub fn factorial(number: f64) -> Result<f64, String> {
    // This is the extended factorial function to be exact the gamma function, it works with non
    // possitive integers.
    // https://mathworld.wolfram.com/GammaFunction.html
    if number == 0.0 {
        Ok(1.0)
    } else if number > 0.0 {
        let result = factorial(number - 1.0)?;
        Ok(number * result)
    } else {
        Err(String::from("Values smaller 0 aren't implemented yet"))
    }
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
