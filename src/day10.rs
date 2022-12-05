use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_1() -> i64 {
    let jolts = include_str!("../input/day10.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<_>>();

    let (mut prev, mut count_1, mut count_3) = (0, 0, 1);
    for x in jolts {
        match x - prev {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => {}
        }
        prev = x;
    }

    count_1 * count_3
}

pub fn solve_2() -> i64 {
    let mut jolts = include_str!("../input/day10.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    jolts.push(0);
    jolts.push(jolts.iter().max().unwrap() + 3);

    // Build a Directed Acyclic Graph (DAG).
    let mut dag = HashMap::new();
    for x in &jolts {
        dag.insert(
            *x,
            (x + 1..x + 4)
                .filter(|y| jolts.contains(y))
                .collect::<Vec<_>>(),
        );
    }

    let mut cache: HashMap<i64, i64> = HashMap::new();
    solver(&dag, 0, &mut cache)
}

fn solver(dag: &HashMap<i64, Vec<i64>>, start: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if let Some(value) = cache.get(&start) {
        return *value;
    }
    let neighbors = dag.get(&start).unwrap();
    if neighbors.is_empty() {
        return 1;
    }
    let value = neighbors
        .iter()
        .map(|x| solver(dag, *x, cache))
        .sum::<i64>();
    cache.insert(start, value);
    value
}
