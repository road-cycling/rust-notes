
// Chapter 1 exercise.

use std::io;
use std::collections::HashMap;

pub(crate) fn start_fib() {

    println!("Please enter a number you would like fibonacci for!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Value entered is not a number.");

    println!("Fib Recursive: {}", fib_recursive(input));
    println!("Fib Memo: {}", fib_r_memo(input));
    println!("Fib Iter: {}", fib_iter(input));
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

fn fib_iter(n: u32) -> u32 {

    let mut num1 = 0;
    let mut num2 = 1;
    let mut counter = 0;

    while counter < n {

        let num3 = num1 + num2;
        num1 = num2;
        num2 = num3;

        counter += 1
    }

    return num2;
}

fn fib_r_memo(n: u32) -> u32 {

    let mut levels: HashMap<u32, u32> = HashMap::new();

    fn fib_r_memo_helper(memo: &mut HashMap<u32, u32>, nacci: u32) -> u32 {

        match memo.get(&nacci) {
            Some(computed_value) => *computed_value,
            None => {
                let result = match nacci {
                    0 => 1,
                    1 => 1,
                    _ => fib_r_memo_helper(memo, nacci - 1) + fib_r_memo_helper(memo, nacci - 2)
                };

                memo.insert(nacci, result);
                result
            }
        }
    }

    return fib_r_memo_helper(&mut levels, n)
}
