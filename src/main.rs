#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name");
    let mut name = String::new();
    let greeting = "Hello, what is your name?";
    io::stdin()
        .read_line(&mut name)
        .expect("Did not enter a correct string");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
