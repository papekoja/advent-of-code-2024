use std::collections::HashSet;

pub fn pt2() -> usize {
    let map: Vec<Vec<char>> = include_str!("input.in")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut pos: (i32, i32) = (0, 0);
    let mut start_pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    let mut visited = HashSet::new();
    for (ridx, r) in map.iter().enumerate() {
        for (cidx, c) in r.iter().enumerate() {
            if *c == '^' {
                pos = (ridx as i32, cidx as i32);
                start_pos = (ridx as i32, cidx as i32);
            };
        }
    }

    let mut res = 0;
    for (ridx, r) in map.iter().enumerate() {
        for (cidx, &c) in r.iter().enumerate() {
            if (ridx as i32, cidx as i32) == start_pos || c == '#' {
                continue;
            }
            dir = (-1, 0);
            visited.clear();
            pos = start_pos;
            let mut temp_map = map.to_vec();
            temp_map[ridx][cidx] = '#';

            while !visited.contains(&(pos, dir)) {
                let mut next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                visited.insert((pos, dir));
                if next_pos.0 >= 0
                    && next_pos.0 < temp_map.len() as i32
                    && next_pos.1 >= 0
                    && next_pos.1 < temp_map[0].len() as i32
                {
                    while temp_map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                        dir = get_new_dir(dir);
                        next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    }
                    pos = next_pos;

                    if visited.contains(&(pos, dir)) {
                        res += 1;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    res
}

fn get_new_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("Invalid direction"),
    }
}
