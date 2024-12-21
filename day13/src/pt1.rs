use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

pub fn pt1() -> i32 {
    let re =
        Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)|Prize: X=(?<px>\d+), Y=(?<py>\d+)")
            .unwrap();
    include_str!("input.in").split("\n\n").fold(0, |acc, text| {
        let mut caps = re.captures_iter(text);
        let a_match = caps.next().unwrap();
        let b_match = caps.next().unwrap();
        let p_match = caps.next().unwrap();
        let mut solver = Solver::new(
            a_match["x"].parse::<i32>().unwrap(),
            a_match["y"].parse::<i32>().unwrap(),
            b_match["x"].parse::<i32>().unwrap(),
            b_match["y"].parse::<i32>().unwrap(),
            p_match["px"].parse::<i32>().unwrap(),
            p_match["py"].parse::<i32>().unwrap(),
        );
        acc + solver.solve()
    })
}

struct Solver {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    px: i32,
    py: i32,
    dp: HashMap<(i32, i32), i32>,
}

impl Solver {
    fn new(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> Self {
        Self {
            ax,
            ay,
            bx,
            by,
            px,
            py,
            dp: HashMap::new(),
        }
    }
    fn solve(&mut self) -> i32 {
        let ans = self.f(self.px, self.py);
        if ans < 1000 {
            return ans;
        } else {
            return 0;
        }
    }

    fn f(&mut self, x: i32, y: i32) -> i32 {
        if self.dp.contains_key(&(x, y)) {
            return *self.dp.get(&(x, y)).unwrap();
        }
        if x < 0 || y < 0 {
            return 10000;
        }
        if x == 0 && y == 0 {
            return 0;
        }
        let res = min(
            3 + self.f(x - self.ax, y - self.ay),
            1 + self.f(x - self.bx, y - self.by),
        );
        self.dp.insert((x, y), res);
        res
    }
}
