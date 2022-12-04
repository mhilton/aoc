// cargo run < ../input

#[macro_use]
extern crate lalrpop_util;

use std::fmt::Display;
use std::io::stdin;

lalrpop_mod!(pub parse);

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    lines
        .map(|s| parse::PairParser::new().parse(&s).unwrap())
        .map(|p| if p.0.overlaps(p.1) { 1 } else { 0 })
        .sum::<usize>()
}

#[derive(Clone, Copy, Debug)]
pub struct SectionRange {
    start: isize,
    end: isize,
}

impl SectionRange {
    pub fn envelops(&self, r: SectionRange) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    pub fn overlaps(&self, r: SectionRange) -> bool {
        !(self.end < r.start || self.start > r.end)
    }
}
