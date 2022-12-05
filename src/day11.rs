use itertools::iproduct;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point2D {
    y: i64,
    x: i64,
}

impl Point2D {
    fn new(y: i64, x: i64) -> Point2D {
        Point2D { y, x }
    }

    fn neighbors(&self) -> Vec<Point2D> {
        vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|&(dy, dx)| Point2D::new(self.y + dy, self.x + dx))
        .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
struct Map2D {
    map: HashMap<Point2D, char>,
    height: i64,
    width: i64,
}

impl Map2D {
    fn new(height: i64, width: i64) -> Map2D {
        Map2D {
            map: HashMap::new(),
            height,
            width,
        }
    }

    fn _print(&self) {
        for (y, x) in iproduct!(0..self.height, 0..self.width) {
            print!("{}", self.map.get(&Point2D::new(y, x)).unwrap());
            if x == self.width - 1 {
                println!();
            }
        }
    }

    fn get_neighbors_1(&self, p: &Point2D) -> Vec<char> {
        p.neighbors()
            .iter()
            .filter(|&p| (0..self.height).contains(&p.y) && (0..self.width).contains(&p.x))
            .map(|p| *self.map.get(p).unwrap())
            .collect()
    }

    fn get_neighbors_2(&self, p: &Point2D) -> Vec<char> {
        let mut neighbors = vec![];
        let deltas = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (dy, dx) in deltas {
            let mut found = None;
            let mut k = 1;
            loop {
                let (dy, dx) = (dy * k, dx * k);
                if !(0..self.height).contains(&(p.y + dy))
                    || !(0..self.width).contains(&(p.x + dx))
                    || found.is_some()
                {
                    break;
                }
                let value = self.map.get(&Point2D::new(p.y + dy, p.x + dx)).unwrap();
                if *value == 'L' || *value == '#' {
                    found = Some(*value);
                }
                k += 1;
            }
            if let Some(value) = found {
                neighbors.push(value);
            }
        }
        neighbors
    }
}

fn build_input_map() -> Map2D {
    let input = include_str!("../input/day11.txt");
    let mut map = Map2D::new(
        input.lines().count() as i64,
        input.lines().next().unwrap().chars().count() as i64,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.map.insert(Point2D::new(y as i64, x as i64), c);
        }
    }
    map
}

pub fn solve_1() -> usize {
    let mut map = build_input_map();

    loop {
        let mut changed = false;
        let map_copy = map.clone();

        for (y, x) in iproduct!(0..map.height, 0..map.width) {
            let point = Point2D::new(y, x);
            let neighbors = map_copy.get_neighbors_1(&point);

            match map_copy.map.get(&point).unwrap() {
                'L' => {
                    if neighbors.iter().all(|c| *c == 'L' || *c == '.') {
                        map.map.insert(point, '#');
                        changed = true;
                    }
                }
                '#' => {
                    if neighbors.iter().filter(|c| **c == '#').count() >= 4 {
                        map.map.insert(point, 'L');
                        changed = true;
                    }
                }
                _ => {}
            }
        }

        if !changed {
            break;
        }
    }

    map.map.values().filter(|c| **c == '#').count()
}

pub fn solve_2() -> usize {
    let mut map = build_input_map();

    loop {
        let mut changed = false;
        let map_copy = map.clone();

        for (y, x) in iproduct!(0..map.height, 0..map.width) {
            let point = Point2D::new(y, x);
            let neighbors = map_copy.get_neighbors_2(&point);

            match map_copy.map.get(&point).unwrap() {
                'L' => {
                    if neighbors.iter().all(|c| *c == 'L') {
                        map.map.insert(point, '#');
                        changed = true;
                    }
                }
                '#' => {
                    if neighbors.iter().filter(|c| **c == '#').count() >= 5 {
                        map.map.insert(point, 'L');
                        changed = true;
                    }
                }
                _ => {}
            }
        }

        if !changed {
            break;
        }
    }

    map.map.values().filter(|c| **c == '#').count()
}
