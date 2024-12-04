use regex::Regex;

pub fn pt1() -> i32 {
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap().captures_iter(include_str!("input.in")).fold( 0, |acc ,capture| { acc + capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap() })
}