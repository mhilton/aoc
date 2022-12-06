// cargo run < ../input

use std::io::{stdin, Read};

fn main() {
    let mut data = Vec::<u8>::new();
    stdin().read_to_end(&mut data).unwrap();
    for i in 0..data.len() {
        let a = data[i];
        let b = data[i + 1];
        let c = data[i + 2];
        let d = data[i + 3];
        if a != b && a != c && a != d && b != c && b != d && c != d {
            println!("{}", i + 4);
            break;
        }
    }
}
