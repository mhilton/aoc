// cargo run < ../input

use std::fmt::Display;
use std::io::stdin;

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    lines
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            let i = a.find(|c| b.contains(c)).unwrap();
            a[i..].chars().take(1).next().unwrap()
        })
        .map(|c| match c {
            c if 'a' <= c && c <= 'z' => 1 + u32::from(c) - u32::from('a'),
            c if 'A' <= c && c <= 'Z' => 27 + u32::from(c) - u32::from('A'),
            _ => panic!("?"),
        })
        .sum::<u32>()
}
