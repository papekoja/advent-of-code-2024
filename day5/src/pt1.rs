use std::collections::HashMap;

pub fn pt1() -> i32 {
    let mut rules: HashMap<usize, usize> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = vec![];

    let mut sections = include_str!("input.in").split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    rules_section.lines().for_each(|line| {

        let v: Vec<&str> = line.split("|").collect();
        rules.insert(
            v[1].parse::<usize>().unwrap(),
            v[0].parse::<usize>().unwrap(),
        );
    });

    updates_section.lines().for_each(|line| {
        updates.push(
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        )
    });

    println!("rules: {:?}", rules);
    let filtered: Vec<_> = updates
        .iter()
        .filter(|update| {
            let mut seen: Vec<bool> = vec![false; 101];
            let result = update.iter().all(|&n| {
                seen[n] = true;
                let res = match rules.get(&n) {
                    Some(pred) => seen[*pred],
                    None => true,
                };
                println!("returned {} for {} with seen {:?}", res, n, seen);
                res
            });
            println!("Update: {:?}, Seen: {:?}, Result: {}", update, seen, result);
            result
        })
        .collect();
    println!("Filtered: {:?}", filtered);
    let result = filtered.into_iter().fold(0, |acc, update| {
        let mid = update[update.len() / 2] as i32;
        println!("Acc: {}, Middle: {}, Update: {:?}", acc, mid, update);
        acc + mid
    });
    result
}
