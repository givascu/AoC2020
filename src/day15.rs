use std::collections::HashMap;

fn solver(numbers: &[usize], target: usize) -> usize {
    let mut cache = HashMap::new();
    for i in 1..numbers.len() {
        cache.insert(numbers[i - 1], i);
    }

    let mut last_num = *numbers.last().unwrap();
    let mut turn = numbers.len();
    while turn != target {
        let new_num = match cache.get(&last_num) {
            Some(val) => turn - *val,
            None => 0,
        };
        cache.insert(last_num, turn);
        last_num = new_num;
        turn += 1;
    }

    last_num
}

pub fn solve_1() -> usize {
    let numbers = include_str!("../input/day15.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    solver(&numbers, 2020)
}

pub fn solve_2() -> usize {
    let numbers = include_str!("../input/day15.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    solver(&numbers, 30_000_000)
}
