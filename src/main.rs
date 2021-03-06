
mod enigma;

#[macro_use] extern crate maplit;

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .next();

    println!("{:?}", input);

    println!("Hello, world!");
}
