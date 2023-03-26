pub fn factorial(number: u128) -> u128 {
    let mut result: u128 = 1;
    for i in 1..=number {
        result *= i;
    }
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
