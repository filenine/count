// A program to count the number of characters in an input string.
// Started 3 November 2023

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = args[1..].join(" ");
    let output = count(&input);
    println!("{}", output);
}

fn count(input: &str) -> u64 {
    let mut char_count: u64 = 0;
    for c in input.chars() {
        char_count += 1;
    }

    char_count
}