use itertools::Itertools;

pub fn solve_1() -> usize {
    include_str!("../input/day06.txt")
        .split("\n\n")
        .map(|group| group.replace('\n', "").chars().unique().count())
        .sum()
}

pub fn solve_2() -> i64 {
    include_str!("../input/day06.txt")
        .split("\n\n")
        .map(|group| {
            let mut sum = 0;
            // Dummy way to calculate the intersection of all lines.
            for c in [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ] {
                if group.lines().all(|answer| answer.contains(c)) {
                    sum += 1;
                }
            }
            sum
        })
        .sum()
}
