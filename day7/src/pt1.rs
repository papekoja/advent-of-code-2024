
pub fn pt1() -> u128 {
    let data = include_str!("input.in")
    .lines()
    .map(|line| {
        let parts: Vec<_> = line.split(':').collect();
        (parts[0].parse::<usize>().unwrap(), parts[1].split_whitespace().rev().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
    });

    data.fold(0u128, |acc: u128, (value, operands)| {
        if all_operations(value, 0, &mut operands.clone()) {
            acc + value as u128
        } else {
            acc
        }
    })
}

fn all_operations(value: usize, acc: usize, operands: &mut Vec<usize>) -> bool {
    match operands.pop() {
        Some(num) => all_operations(value, acc + num, &mut operands.clone()) || all_operations(value, acc * num, &mut operands.clone()),
        None => acc == value
    }
}