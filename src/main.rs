// cargo run < ../input

use std::collections::HashMap;
use std::fmt::Display;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    println!("{}", run(stdin().lines().map(|s| s.unwrap())));
}

fn run(lines: impl Iterator<Item = String>) -> impl Display {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = head;

    let mut map = HashMap::<Pos, isize>::new();
    map.insert(tail, 1);

    for line in lines {
        let mut it = line.split_whitespace();
        let mv = it.next().unwrap();
        let n = isize::from_str(it.next().unwrap()).unwrap();
        for _ in 0..n {
            match mv {
                "D" => head = head.down(),
                "L" => head = head.left(),
                "R" => head = head.right(),
                "U" => head = head.up(),
                _ => panic!("bad instruction: {}", mv),
            }
            tail = tail.follow(&head);
            map.entry(tail).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    map.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn up(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn follow(&self, h: &Pos) -> Pos {
        match (h.x - self.x, h.y - self.y) {
            (-1, -2) => Pos {
                x: self.x - 1,
                y: self.y - 1,
            },
            (0, -2) => Pos {
                x: self.x,
                y: self.y - 1,
            },
            (1, -2) => Pos {
                x: self.x + 1,
                y: self.y - 1,
            },
            (-2, -1) => Pos {
                x: self.x - 1,
                y: self.y - 1,
            },
            (-1, -1) => *self,
            (0, -1) => *self,
            (1, -1) => *self,
            (2, -1) => Pos {
                x: self.x + 1,
                y: self.y - 1,
            },
            (-2, 0) => Pos {
                x: self.x - 1,
                y: self.y,
            },
            (-1, 0) => *self,
            (0, 0) => *self,
            (1, 0) => *self,
            (2, 0) => Pos {
                x: self.x + 1,
                y: self.y,
            },
            (-2, 1) => Pos {
                x: self.x - 1,
                y: self.y + 1,
            },
            (-1, 1) => *self,
            (0, 1) => *self,
            (1, 1) => *self,
            (2, 1) => Pos {
                x: self.x + 1,
                y: self.y + 1,
            },
            (-1, 2) => Pos {
                x: self.x - 1,
                y: self.y + 1,
            },
            (0, 2) => Pos {
                x: self.x,
                y: self.y + 1,
            },
            (1, 2) => Pos {
                x: self.x + 1,
                y: self.y + 1,
            },
            _ => panic!("lost head"),
        }
    }
}
