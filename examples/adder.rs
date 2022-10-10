use std::io::stdin;
use fe_scratch::add;

fn main() {
    let a = read_int("Enter integer 1 of 2: ");
    println!();

    let b = read_int("Enter integer 2 of 2: ");
    println!();

    println!("They sum to {}.", add(a, b));
}

fn read_int(prompt: &str) -> usize {
    println!("{}", prompt);

    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .unwrap();

    buffer
        .trim()
        .parse::<usize>()
        .unwrap()
}