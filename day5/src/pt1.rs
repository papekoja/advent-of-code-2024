use std::collections::{HashMap, HashSet};

pub fn pt1() -> i32 {
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = vec![];

    let mut sections = include_str!("input.in").split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    rules_section.lines().for_each(|line| {
        let v: Vec<&str> = line.split("|").collect();
        rules
            .entry(v[1].parse::<usize>().unwrap())
            .or_insert(HashSet::new())
            .insert(v[0].parse::<usize>().unwrap());
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
                match rules.get(&n) {
                    Some(predecessors) => predecessors
                        .iter()
                        .all(|pred| seen.contains(pred) || !update.contains(pred)),
                    None => true,
                }
            });
            result
        })
        .collect();
    let result = filtered.into_iter().fold(0, |acc, update| {
        let mid = update[update.len() / 2] as i32;
        acc + mid
    });
    result
}
