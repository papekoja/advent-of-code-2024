use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Reverse;

fn main() {
    let file_path = "../input.in";
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file: {}", err);
        std::process::exit(1);
    });
    let reader = io::BufReader::new(file);

    let mut h1 = BinaryHeap::new();
    let mut h2 = BinaryHeap::new();
    
    reader
    .lines()
    .for_each(|line| {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            h1.push(Reverse(num1));
            h2.push(Reverse(num2));

        }
    });

    let mut res = 0;
    while let (Some(Reverse(num1)), Some(Reverse(num2))) = (h1.pop(), h2.pop()) {
        res += (num1 - num2).abs()
    }

    println!("{}", res)
}
