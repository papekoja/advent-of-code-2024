use std::collections::VecDeque;

pub fn pt1() -> i32 {
    let data: Vec<Vec<char>> = include_str!("input.in")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    println!("{:?}", data);
    0
}

fn get_regions(matrix: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions: Vec<Vec<(usize, usize)>> = vec![];

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let bfs = |start_x: usize, start_y: usize| {
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y));
    };
}
