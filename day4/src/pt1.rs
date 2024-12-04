
pub fn pt1() {
    let data: Vec<Vec<_>> = include_str!("input.in").lines().map(|l| l.chars().collect()).collect();

    let mut res = 0;

    for c in 0..data[0].len() {
        for r in 0..data.len() {
            res += correct(&data, vec![(c, r), (c + 1, r), (c + 2, r), (c + 3, r)]);
            res += correct(&data, vec![(c, r), (c, r + 1), (c, r + 2), (c, r + 3)]);
            res += correct(&data, vec![(c, r), (c + 1, r + 1), (c + 2, r + 2), (c + 3, r + 3)]);
            res += correct(&data, vec![(c, r), (c + 1, r.wrapping_sub(1)), (c + 2, r.wrapping_sub(2)), (c + 3, r.wrapping_sub(3))]);
        }
    }

    println!("{}", res);
    //data.into_iter().for_each(|v| println!("{:?}", v));
}

fn correct(matrix: &Vec<Vec<char>>, v: Vec<(usize, usize)>) -> i32 {
    let word: Vec<char> = "XMAS".chars().collect();
    let mut xarrs: Vec<Vec<char>> = vec![word.clone(), word];
    xarrs[1].reverse();

    if v[3].0 >= matrix[0].len() || v[3].1 >= matrix.len() {
        return 0;
    }

    let u: Vec<char> = v.into_iter().map(|(c, r)| matrix[r][c]).collect();
    if u == xarrs[0] || u == xarrs[1] { 1 } else { 0 }
}