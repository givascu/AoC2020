pub fn solve_2() -> i64 {
    let numbers = include_str!("../input/01.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    -1
}

pub fn solve_1() -> i64 {
    let numbers = include_str!("../input/01.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for x in &numbers {
        for y in &numbers {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    -1
}
