// cargo run < ../input

use std::fmt::Display;
use std::io::{read_to_string, stdin};
use std::str::FromStr;

fn main() {
    println!("{}", run(read_to_string(stdin()).unwrap().as_str()));
}

fn run(input: &str) -> impl Display {
    input
        .split_ascii_whitespace()
        .map(|w| match i64::from_str(w) {
            Ok(v) => v,
            _ => 0,
        })
        .scan(1, |x, v| {
            let y = Some(*x);
            // Value changes at the end of the cycle.
            *x += v;
            y
        })
        .enumerate()
        .filter_map(|(n, x)| {
            // cycle count is 1 based.
            if n % 40 == 19 {
                Some(x * i64::try_from(n + 1).unwrap())
            } else {
                None
            }
        })
        .take(6)
        .sum::<i64>()
}
