use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut count = 0;
    let mut max = 0;
    for line in stdin().lines() {
        let s = line.expect("day01");
        if !s.is_empty() {
            count += isize::from_str(&s).expect("day01");
        } else {
            if count > max {
                max = count
            };
            count = 0;
        };
    }

    println!("{}", max);
}
