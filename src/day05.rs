use itertools::Itertools;

fn bsearch(mut lower: i64, mut upper: i64, sequence: &str) -> i64 {
    for c in sequence.chars() {
        if c == 'F' || c == 'L' {
            upper = (lower + upper) / 2;
        } else if c == 'B' || c == 'R' {
            lower = (lower + upper) / 2 + 1;
        } else {
            unreachable!("Invalid character: '{c}'");
        }
    }
    lower
}

pub fn solve_1() -> i64 {
    include_str!("../input/day05.txt")
        .lines()
        .map(|x| bsearch(0, 127, &x[..7]) * 8 + bsearch(0, 7, &x[7..]))
        .max()
        .unwrap()
}

pub fn solve_2() -> i64 {
    let mut prev_value = i64::MAX;
    for value in include_str!("../input/day05.txt")
        .lines()
        .map(|x| bsearch(0, 127, &x[..7]) * 8 + bsearch(0, 7, &x[7..]))
        .sorted()
    {
        if value - prev_value == 2 {
            return value - 1;
        }
        prev_value = value;
    }
    -1
}
