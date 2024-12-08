use std::collections::{HashMap, HashSet};

pub fn pt1() -> usize {
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
                    let antinode = ((2 * u1).wrapping_sub(v1), (2 * u2).wrapping_sub(v2));
                    if antinode.0 < data.len() && antinode.1 < data[0].len() {
                        antinodes.insert(antinode);
                    }
                }
            });
        });
    });
    antinodes.iter().for_each(|&(v1, v2)| {
        data[v1][v2] = '#';
    });
    antinodes.len()
}
