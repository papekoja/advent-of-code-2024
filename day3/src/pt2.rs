use regex::Regex;

pub fn pt2() -> i32 {
    let mut foo = true;
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap().captures_iter(include_str!("input.in")).fold( 0, |acc ,capture| { 
        if let (Some(capture1), Some(capture2)) = (capture.get(1), capture.get(2)) {
            if foo {
                return acc + capture1.as_str().parse::<i32>().unwrap() * capture2.as_str().parse::<i32>().unwrap()
            }
        } else {
            if &capture[0] == "do()" {
               foo = true;
            } else {
               foo = false;
            }
       }
       acc
    })
}