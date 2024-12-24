use regex::Regex;
use std::i64;

pub fn pt2() -> i64 {
    let re =
        Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)|Prize: X=(?<px>\d+), Y=(?<py>\d+)")
            .unwrap();
    include_str!("input.in").split("\n\n").fold(0, |acc, text| {
        let mut caps = re.captures_iter(text);
        let a_match = caps.next().unwrap();
        let b_match = caps.next().unwrap();
        let p_match = caps.next().unwrap();
        let ax = a_match["x"].parse::<i64>().unwrap();
        let ay = a_match["y"].parse::<i64>().unwrap();
        let bx = b_match["x"].parse::<i64>().unwrap();
        let by = b_match["y"].parse::<i64>().unwrap();
        let px = p_match["px"].parse::<i64>().unwrap() + 10_000_000_000_000;
        let py = p_match["py"].parse::<i64>().unwrap() + 10_000_000_000_000;

        let det = ax * by - ay * bx;
        let b = by * px - bx * py;
        let a = ax * py - ay * px;

        if det == 0 || a % det != 0 || b % det != 0 {
            return acc;
        }

        return acc + (a + 3 * b) / det;
    })
}
