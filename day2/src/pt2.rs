pub fn pt2() -> usize {
    include_str!("input.in")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|nums| {
            if is_safe(nums) {
                true
            } else {
                (0..nums.len()).any(|i| {
                    let mut modified = nums.to_vec();
                    modified.remove(i);
                    is_safe(&modified)
                })
            }
        })
        .count()
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for window in nums.windows(2) {
        let distance = (window[0] - window[1]).abs();
        if distance > 3 {
            return false;
        }
        if window[0] >= window[1] {
            increasing = false;
        }
        if window[0] <= window[1] {
            decreasing = false;
        }
    }
    increasing || decreasing
}
