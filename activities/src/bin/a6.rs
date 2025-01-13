// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    display_countdown();
}

fn display_countdown() {
    let mut num: i32 = 5;
    while num > 0 {
        println!("{}", num);
        num -= 1;
    }
    println!("done!");
}
