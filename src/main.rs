// cargo run < ../input

use std::fmt::Display;
use std::io::stdin;

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    "!!TODO!!"
}
