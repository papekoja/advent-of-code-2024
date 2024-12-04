pub fn pt2() {
    let data: Vec<Vec<_>> = include_str!("input.in").lines().map(|l| l.chars().collect()).collect();

    let mut res = 0;

    for c in 0..data[0].len() {
        for r in 0..data.len() {
            let r1 = correct(&data, vec![(c.wrapping_sub(1), r.wrapping_sub(1)), (c, r), (c + 1, r + 1)]);
            let r2 = correct(&data, vec![(c + 1, r.wrapping_sub(1)), (c, r), (c.wrapping_sub(1), r + 1)]);
            if r1 > 0 && r2 > 0 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

fn correct(matrix: &Vec<Vec<char>>, v: Vec<(usize, usize)>) -> i32 {
    let w1: Vec<char> = vec!['M', 'A', 'S'];
    let w2: Vec<char> = vec!['S', 'A', 'M'];

    for &(c, r) in &v {
        if c >= matrix[0].len() || c >= matrix.len() || r >= matrix[0].len() || r >= matrix.len() {
            return 0;
        }
    }

    let u: Vec<char> = v.into_iter().map(|(c, r)| matrix[r][c]).collect();
    if u == w1 || u == w2 { 1 } else { 0 }
}