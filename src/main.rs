// cargo run < ../input

use std::collections::HashMap;
use std::fmt::Display;
use std::io::stdin;

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    let lines = lines.collect::<Vec<_>>();
    let blank = lines.iter().position(|s| s.is_empty()).unwrap();
    let (diag_lines, moves) = lines.split_at(blank);

    let mut diag = diag_lines.iter().collect::<Vec<_>>();
    diag.reverse();
    let (label_lines, layout) = diag.split_at(1);
    let labels = label_lines[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut stacks = HashMap::<String, Vec<char>>::new();
    for level in layout {
        for (i, c) in level
            .chars()
            .enumerate()
            .filter(|&(i, c)| i % 4 == 1 && c != ' ')
            .map(|(i, c)| (i / 4, c))
        {
            stacks
                .entry(labels[i].clone())
                .and_modify(|s| s.push(c))
                .or_insert(vec![c]);
        }
    }

    for line in moves {
        if line.is_empty() {
            continue;
        }
        let words = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let count: isize = words[1].parse().unwrap();
        let mut temp = Vec::<char>::new();
        stacks.entry(words[3].to_string()).and_modify(|v| {
            for _ in 0..count {
                temp.push(v.pop().unwrap())
            }
        });
        stacks.entry(words[5].to_string()).and_modify(|v| {
            for _ in 0..count {
                v.push(temp.pop().unwrap())
            }
        });
    }

    labels
        .into_iter()
        .map(|s| stacks[&s].last().unwrap())
        .collect::<String>()
}
