#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn rotate_ship(direction: &Direction, degrees: i32) -> Direction {
    match degrees {
        90 => match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
        180 => match direction {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        },
        270 => match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        },
        _ => unreachable!(),
    }
}

pub fn solve_1() -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut facing = Direction::East;

    for line in include_str!("../input/day12.txt").lines() {
        let cmd = line.chars().next().unwrap();
        let arg = line[1..].parse::<i32>().unwrap();

        match cmd {
            'N' => y += arg,
            'S' => y -= arg,
            'E' => x += arg,
            'W' => x -= arg,
            'R' => facing = rotate_ship(&facing, arg),
            'L' => facing = rotate_ship(&facing, 360 - arg),
            'F' => match facing {
                Direction::North => y += arg,
                Direction::South => y -= arg,
                Direction::East => x += arg,
                Direction::West => x -= arg,
            },
            _ => unreachable!(),
        }
    }

    x.abs() + y.abs()
}

fn rotate_waypoint(dx: i32, dy: i32, degrees: i32) -> (i32, i32) {
    match degrees {
        90 => (dy, -dx),
        180 => (-dx, -dy),
        270 => (-dy, dx),
        _ => unreachable!(),
    }
}

pub fn solve_2() -> i32 {
    let (mut x, mut y) = (0, 0); // ship's absolute position
    let (mut dx, mut dy) = (10, 1); // waypoint's relative position to ship

    for line in include_str!("../input/day12.txt").lines() {
        let cmd = line.chars().next().unwrap();
        let arg = line[1..].parse::<i32>().unwrap();

        match cmd {
            'N' => dy += arg,
            'S' => dy -= arg,
            'E' => dx += arg,
            'W' => dx -= arg,
            'R' => (dx, dy) = rotate_waypoint(dx, dy, arg),
            'L' => (dx, dy) = rotate_waypoint(dx, dy, 360 - arg),
            'F' => {
                x += arg * dx;
                y += arg * dy;
            }
            _ => unreachable!(),
        }
    }

    x.abs() + y.abs()
}
