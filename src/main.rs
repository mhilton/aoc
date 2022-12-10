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
        .map(|(n, x)| {
            let mut s = String::new();
            let n = i64::try_from(n).unwrap() % 40;
            if x == n - 1 || x == n || x == n + 1 {
                s.push('#')
            } else {
                s.push('.')
            };
            if n == 39 {
                s.push('\n')
            };
            s
        })
        .collect::<String>()
}
