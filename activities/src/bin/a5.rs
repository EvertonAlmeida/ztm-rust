// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    display_numbers();
}

fn display_numbers() {
    let mut num: i32 = 1;
    loop {
        println!("{}", num);
        num += 1;
        if num > 4 {
            break;
        }
    }
}
