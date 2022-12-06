// cargo run < ../input

use std::io::{stdin, Read};

fn main() {
    let mut data = Vec::<u8>::new();
    stdin().read_to_end(&mut data).unwrap();
    for i in 0..(data.len() - 14) {
        if unique(&data[i..i + 14]) {
            println!("{}", i + 14);
            break;
        }
    }
}

fn unique(data: &[u8]) -> bool {
    if data.len() == 1 {
        true
    } else {
        let x = data[0];
        for y in &data[1..] {
            if x == *y {
                return false;
            }
        }
        unique(&data[1..])
    }
}
