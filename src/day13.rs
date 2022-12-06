use num_integer::lcm;

pub fn solve_1() -> i32 {
    let mut input = include_str!("../input/day13.txt").lines();
    let n = input.next().unwrap().parse::<i32>().unwrap();
    let ids = input
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut start = n + 1;
    loop {
        if let Some(id) = ids.iter().find(|&&x| start % x == 0) {
            return *id * (start - n);
        }
        start += 1;
    }
}

pub fn solve_2() -> i64 {
    let ids = include_str!("../input/day13.txt")
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| {
            if x == "x" {
                0
            } else {
                x.parse::<i64>().unwrap()
            }
        })
        .collect::<Vec<_>>();

    // Algorithm: advance the time index by the first bus' interval until the
    // second bus is in the desired position relative to the first. From now on,
    // the two buses move in lockstep positions relative to each other, i.e.
    // they move as one, each LCM(bus1.interval, bus2.interval) time units.
    // Advance the time index by this new step until the third bus is in the
    // desired position. At that point, all three buses can be moved in lockstep
    // positions each LCM(bus1.interval, bus2.interval, bus3.interval) time
    // units. Repeat this process until the final bus falls into the desired
    // position, and that time index is the answer.

    let mut pivot = ids[0];
    let mut target = pivot;

    for (idx, &next) in ids.iter().enumerate().skip(1) {
        // Skip 'x' buses.
        if next == 0 {
            continue;
        }
        // Always validate relatively to the first bus.
        while (target + (idx as i64)) % next != 0 {
            target += pivot;
        }
        // Break if the nth bus condition is met.
        if idx == ids.len() - 1 {
            break;
        }
        // The step required so the (n-1) buses and the nth one move in lockstep.
        pivot = lcm(pivot, next);
        // Merge the nth bus into previous (n-1) buses for the next iteration.
        target += pivot;
    }

    target
}
