use std::io;

fn start_fib() {

    println!("Please enter a number you would like fibonacci for!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Value entered is not a number.");

    println!("Fib Recursive: {}", fib_recursive(input))
}

// fib_recursive - calculates the nth sequence of the fibonacci
// sequence with recursion (no memoization)
fn fib_recursive(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
