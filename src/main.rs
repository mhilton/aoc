// cargo run < ../input

use std::fmt::Display;
use std::io::stdin;

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    lines
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let i = chunk[0]
                .find(|c| chunk[1].contains(c) && chunk[2].contains(c))
                .unwrap();
            chunk[0][i..].chars().take(1).next().unwrap()
        })
        .map(|c| match c {
            c if 'a' <= c && c <= 'z' => 1 + u32::from(c) - u32::from('a'),
            c if 'A' <= c && c <= 'Z' => 27 + u32::from(c) - u32::from('A'),
            _ => panic!("?"),
        })
        .sum::<u32>()
}
