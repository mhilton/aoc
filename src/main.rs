use std::io::stdin;

fn main() {
    let score: isize = stdin()
        .lines()
        .map(|s| match s.unwrap().as_str() {
            // As there are only 9 valid strings precompute the outcomes.
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            // Follow ed's UI and simply invite the user to reflect on their failure.
            _ => panic!("?"),
        })
        .sum();
    println!("{score}");
}
