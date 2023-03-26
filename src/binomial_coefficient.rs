use crate::factorials::factorial;

pub fn binomial_coefficient(n: i128, k: i128) -> i128 {
    // This function calculates the binomial coefficient
    // n choose k
    // n! / (k! * (n - k)!)
    // https://mathworld.wolfram.com/BinomialCoefficient.html
    factorial(n) / (factorial(k) * factorial(n - k))
}
