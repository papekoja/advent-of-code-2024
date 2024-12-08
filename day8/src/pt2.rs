use std::collections::{HashMap, HashSet};

pub fn pt2() -> usize {
    let mut data: Vec<Vec<char>> = include_str!("input.in")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let mut antenna_coordinates: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (ridx, row) in data.iter().enumerate() {
        for (cidx, &elem) in row.iter().enumerate() {
            if elem != '.' {
                antenna_coordinates
                    .entry(elem)
                    .or_default()
                    .push((ridx, cidx));
            }
        }
    }
    antenna_coordinates.iter().for_each(|(_, coordinates)| {
        coordinates.iter().for_each(|&(u1, u2)| {
            coordinates.iter().for_each(|&(v1, v2)| {
                if u1 != v1 && u2 != v2 {
                    let distance = (v1 as i32 - u1 as i32, v2 as i32 - u2 as i32);
                    let mut antinode = (u1 as i32 + distance.0, u2 as i32 + distance.1);
                    while antinode.0 >= 0
                        && antinode.0 < data.len() as i32
                        && antinode.1 >= 0
                        && antinode.1 < data[0].len() as i32
                    {
                        antinodes.insert((antinode.0 as usize, antinode.1 as usize));
                        antinode = (antinode.0 + distance.0, antinode.1 + distance.1);
                    }
                }
            });
        });
    });
    antinodes.len()
}
