use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point2D {
    y: usize,
    x: usize,
}

impl Point2D {
    fn new(y: usize, x: usize) -> Point2D {
        Point2D { y, x }
    }
}

struct Map2D<T> {
    map: HashMap<Point2D, T>,
    height: usize,
    width: usize,
}

impl<T> Deref for Map2D<T> {
    type Target = HashMap<Point2D, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> DerefMut for Map2D<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<T> Map2D<T> {
    fn new(height: usize, width: usize) -> Map2D<T> {
        Map2D {
            map: HashMap::new(),
            height,
            width,
        }
    }
}

fn build_input_map() -> Map2D<char> {
    let mut map = Map2D::new(0, 0);
    for (y, line) in include_str!("../input/day03.txt").lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            map.insert(Point2D::new(y, x), val);
        }
        map.height += 1;
        map.width = line.chars().count();
    }
    map
}

fn solve_slope(map: &Map2D<char>, dx: usize, dy: usize) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut tree_count = 0;

    while y < map.height - 1 {
        y += dy;
        x = (x + dx) % map.width;
        if *map.get(&Point2D::new(y, x)).unwrap() == '#' {
            tree_count += 1;
        }
    }

    tree_count
}

pub fn solve_1() -> i64 {
    let map = build_input_map();
    solve_slope(&map, 3, 1)
}

pub fn solve_2() -> i64 {
    let map = build_input_map();
    solve_slope(&map, 1, 1)
        * solve_slope(&map, 3, 1)
        * solve_slope(&map, 5, 1)
        * solve_slope(&map, 7, 1)
        * solve_slope(&map, 1, 2)
}
