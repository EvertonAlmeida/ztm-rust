// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let first_name: &str = display_first_name("John");
    let last_name: &str = display_last_name("Doe");
    println!("My name is {} {}", first_name, last_name);
}

fn display_first_name(name: &str) -> &str {
    name
}

fn display_last_name(last_name: &str) -> &str {
    &last_name
}
