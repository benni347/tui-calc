pub fn factorial(number: i128) -> i128 {
    // This function calculates the factorial of a number
    // https://mathworld.wolfram.com/Factorial.html
    let result: i128;
    result = multifactorial(number, 1);
    result
}

pub fn hyperfactorial(number: u128) -> u128 {
    // This function calculates the hyperfactorial of a number
    // https://mathworld.wolfram.com/Hyperfactorial.html
    let mut result: u128 = 1;
    for i in 1..=number {
        result *= i.pow(i as u32);
    }
    // TODO: If it is smaller than 1 return error must be an int bigger equal 1
    result
}

pub fn multifactorial(number: i128, amount: i128) -> i128 {
    // This function calculates the multifactorial of a number
    // https://mathworld.wolfram.com/Multifactorial.html
    let result: i128;
    if -amount < number && number <= 0 {
        result = 1;
    } else if number > 0 {
        return number * multifactorial(number - amount, amount);
    } else {
        result = 0;
    }
    return result;
}
