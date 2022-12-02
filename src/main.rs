use std::io::stdin;

const LOSE: isize = 0;
const DRAW: isize = 3;
const WIN: isize = 6;

const ROCK: isize = 1;
const PAPER: isize = 2;
const SCISSORS: isize = 3;

fn main() {
    let score: isize = stdin()
        .lines()
        .map(|s| match s.unwrap().as_str() {
            // As there are only 9 valid strings precompute the outcomes.
            "A X" => LOSE + SCISSORS,
            "A Y" => DRAW + ROCK,
            "A Z" => WIN + PAPER,
            "B X" => LOSE + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,
            "C X" => LOSE + PAPER,
            "C Y" => DRAW + SCISSORS,
            "C Z" => WIN + ROCK,
            // Follow ed's UI and simply invite the user to reflect on their failure.
            _ => panic!("?"),
        })
        .sum();
    println!("{score}");
}
