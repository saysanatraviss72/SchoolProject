fn main() {
    // Example Rust code to calculate the factorial of a number
    let n = 5;
    let result = factorial(n);
    println!("Factorial of {}", n.to_string());
    println!("Result: {}", result);
}

/// Calculates the factorial of a given number.
///
/// # Parameters
/// * `n` - The number for which to calculate the factorial.
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}
