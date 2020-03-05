fn main() {
    let condition = true;
    // Return from arms must all the same type
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
