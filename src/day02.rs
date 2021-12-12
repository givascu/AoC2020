pub fn solve_2() -> i64 {
    include_str!("../input/02.txt")
        .lines()
        .filter(|&line| {
            let (policy, password) = line.split_once(':').unwrap();
            let (range, c) = policy.split_once(' ').unwrap();
            let (i1, i2) = range.split_once('-').unwrap();
            let (i1, i2) = (i1.parse::<usize>().unwrap(), i2.parse::<usize>().unwrap());

            let c = c.chars().next().unwrap();
            let password = password.trim();
            let c1 = password.chars().nth(i1 - 1).unwrap();
            let c2 = password.chars().nth(i2 - 1).unwrap();

            (c1 == c || c2 == c) && !(c1 == c && c2 == c)
        })
        .count() as i64
}

pub fn solve_1() -> i64 {
    include_str!("../input/02.txt")
        .lines()
        .filter(|&line| {
            let (policy, password) = line.split_once(':').unwrap();
            let (range, c) = policy.split_once(' ').unwrap();
            let (min, max) = range.split_once('-').unwrap();
            let (min, max) = (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap());

            let c = c.chars().next().unwrap();
            let password = password.trim();

            (min..max + 1).contains(&password.chars().filter(|x| *x == c).count())
        })
        .count() as i64
}
