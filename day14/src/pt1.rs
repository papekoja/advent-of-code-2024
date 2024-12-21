use regex::Regex;

pub fn pt1() -> usize {
    let width = 101;
    let height = 103;
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    let data: Vec<_> = include_str!("input.in")
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

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    data.iter().for_each(|(pos, vel)| {
        let end_px = (pos.0 + vel.0 * 100).rem_euclid(width);
        let end_py = (pos.1 + vel.1 * 100).rem_euclid(height);
        let mid_x = width / 2;
        let mid_y = height / 2;
        if end_px < mid_x && end_py < mid_y {
            q1 += 1;
        } else if end_px > mid_x && end_py < mid_y {
            q2 += 1;
        } else if end_px < mid_x && end_py > mid_y {
            q3 += 1;
        } else if end_px > mid_x && end_py > mid_y {
            q4 += 1;
        }
    });

    q1 * q2 * q3 * q4
}
