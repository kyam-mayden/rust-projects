use std::io;

fn main() {
    let n = loop {
        println!("I want the ___th fibonacci number");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break guess
    };

    let fib = fib(n);
    println!("The {}th fibonacci number is {}", n, fib)
}

fn fib(number: i32) -> i32 {
    if number <= 1 {
        return number;
    }

    return fib(number - 1) + fib(number - 2);
}
