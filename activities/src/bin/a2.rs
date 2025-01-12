// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let a: i32 = 5;
    let b: i32 = 7;
    let sum: i32 = add_two_numbers(a, b);
    display_sum(a, b, sum);
}

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn display_sum(a: i32, b: i32, sum: i32) {
    println!("The sum of {} and {} is {:?}", a, b, sum);
}
