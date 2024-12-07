use std::collections::{HashMap, HashSet, VecDeque};

pub fn pt2() -> i32 {
    // rules and predecessors are inverse of eachother
    let mut predecessors: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = vec![];

    let mut sections = include_str!("input.in").split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    rules_section.lines().for_each(|line| {
        let v: Vec<&str> = line.split("|").collect();
        predecessors
            .entry(v[1].parse::<usize>().unwrap())
            .or_insert(HashSet::new())
            .insert(v[0].parse::<usize>().unwrap());

        rules
            .entry(v[0].parse::<usize>().unwrap())
            .or_insert(HashSet::new())
            .insert(v[1].parse::<usize>().unwrap());
    });

    updates_section.lines().for_each(|line| {
        updates.push(
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        )
    });

    let filtered: Vec<_> = updates
        .iter()
        .filter(|update| {
            let mut seen = HashSet::new();
            let result = update.iter().all(|&n| {
                seen.insert(n);
                match predecessors.get(&n) {
                    Some(predecessors) => predecessors
                        .iter()
                        .all(|pred| seen.contains(pred) || !update.contains(pred)),
                    None => true,
                }
            });
            !result
        })
        .collect();

    let sorted: Vec<_> = filtered
        .iter()
        .map(|&update| topo_sort(&rules, update))
        .collect();

    let result = sorted.into_iter().fold(0, |acc, update| {
        let mid = update[update.len() / 2] as i32;
        acc + mid
    });
    result
}

fn topo_sort(rules: &HashMap<usize, HashSet<usize>>, update: &Vec<usize>) -> Vec<usize> {
    let mut in_degree = HashMap::new();
    for node in update {
        in_degree.entry(node).or_insert(0);
        if let Some(neighbours) = rules.get(node) {
            neighbours.iter().for_each(|neighbour| {
                if update.contains(neighbour) {
                    *in_degree.entry(neighbour).or_insert(0) += 1
                }
            });
        }
    }

    let mut queue = VecDeque::new();
    in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .for_each(|(&node, _)| queue.push_back(node));

    let mut result = vec![];
    while let Some(node) = queue.pop_front() {
        result.push(*node);
        if let Some(neighbours) = rules.get(&node) {
            neighbours.iter().for_each(|neighbour| {
                if update.contains(neighbour) {
                    let deg = in_degree.get_mut(neighbour).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbour);
                    }
                }
            });
        }
    }

    result
}
