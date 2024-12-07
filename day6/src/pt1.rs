pub fn pt1() -> usize {
    let map: Vec<Vec<char>> = include_str!("input.in").lines().map(|s| s.chars().collect()).collect();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (ridx, r) in map.iter().enumerate() {
        for (cidx, c) in r.iter().enumerate() {
            if *c == '^' {
                pos = (ridx as i32, cidx as i32);
                visited[ridx][cidx] = true;
            };
        }
    }
    
    loop {
        let mut next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if next_pos.0 >= 0 && next_pos.0 < map.len() as i32 && next_pos.1 >= 0 && next_pos.1 < map[0].len() as i32 {
            if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                dir = get_new_dir(dir);
                next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
            pos = next_pos;
            visited[pos.0 as usize][pos.1 as usize] = true;
        } else {
            break;
        }
    }
    
    visited
        .iter()
        .flat_map(|row| row)
        .filter(|&&value| value)
        .count()
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
