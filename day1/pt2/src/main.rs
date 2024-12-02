use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "../input.in";
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file: {}", err);
        std::process::exit(1);
    });
    let reader = io::BufReader::new(file);

    let mut v1 = vec![];
    let mut v2 = vec![];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            v1.push(num1);
            v2.push(num2);
        }
    });

    let res = v1.into_iter().fold(0, |acc, num| {
        acc + num * (v2.clone().into_iter().filter(|&x| x == num).count() as i32)
    });

    println!("{}", res)
}
