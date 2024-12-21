use std::usize;

use regex::Regex;

pub fn pt2() -> usize {
    // width is 0 dim of vec and height is 1 dim
    let width = 101;
    let height = 103;
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    let mut data: Vec<_> = include_str!("input.in")
        .lines()
        .filter_map(|line| {
            if let Some(caps) = re.captures(line) {
                let px: i32 = caps[1].parse().unwrap();
                let py: i32 = caps[2].parse().unwrap();
                let vx: i32 = caps[3].parse().unwrap();
                let vy: i32 = caps[4].parse().unwrap();

                Some(((px, py), (vx, vy)))
            } else {
                None
            }
        })
        .collect();

    let dirs = [
        (1, 0),   // Right
        (0, 1),   // Down
        (-1, 0),  // Left
        (0, -1),  // Up
        (1, 1),   // Down-Right
        (-1, 1),  // Down-Left
        (-1, -1), // Up-Left
        (1, -1),  // Up-Right
    ];
    let mut res = 0;
    for _ in 0..10000 {
        data = data
            .iter()
            .map(|(pos, vel)| {
                let end_px = (pos.0 + vel.0).rem_euclid(width);
                let end_py = (pos.1 + vel.1).rem_euclid(height);
                ((end_px, end_py), vel.clone())
            })
            .collect::<Vec<_>>();
        let numer_adj = data.iter().fold(0, |acc, (pos, _)| {
            let found_nb = dirs.iter().any(|dir| {
                let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
                data.iter().any(|robot| robot.0 == neighbor)
            });
            if found_nb {
                acc + 1
            } else {
                acc
            }
        });
        res += 1;
        if numer_adj > 300 {
            print_tree(&data);
        }
    }

    res
}

fn print_tree(positions: &Vec<((i32, i32), (i32, i32))>) {
    let width = 101;
    let height = 103;

    let mut matrix = vec![vec![0; width as usize]; height as usize];

    for ((px, py), _) in positions {
        matrix[*py as usize][*px as usize] += 1; // Count robots at each position
    }

    for row in matrix {
        let row_str: String = row
            .iter()
            .map(|&count| {
                if count > 0 {
                    char::from_digit(count as u32, 10).unwrap_or('#')
                } else {
                    '.'
                }
            })
            .collect();
        println!("{}", row_str);
    }
}
