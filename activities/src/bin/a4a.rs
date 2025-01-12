// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let value: bool = true;
    println!("{}", is_true(value));
}

fn is_true(value: bool) -> String {
    match value {
        true => "it's true".to_string(),
        false => "it's false".to_string(),
    }
}
