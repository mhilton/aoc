use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut count = 0;
    let mut carrying = Vec::<isize>::new();
    for line in stdin().lines() {
        let s = line.expect("day01");
        if !s.is_empty() {
            count += isize::from_str(&s).expect("day01");
        } else {
            carrying.push(count);
            count = 0;
        };
    }
    carrying.sort_unstable();
    let sum: isize = (&carrying[carrying.len() - 3..]).into_iter().sum();
    println!("{}", sum);
}
