// This function takes two numbers and returns their sum.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// This function takes a number and returns its square.
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // Call the add function with 3 and 4, and assign the result to a variable.
    let sum = add(3, 4);

    // Print the value of the sum variable to the console.
    println!("The sum is {}", sum);

    // Call the square function with 5, and assign the result to a variable.
    let squared = square(5);

    // Print the value of the squared variable to the console.
    println!("The squared number is {}", squared);
}
