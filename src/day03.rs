use std::collections::HashMap;

struct Grid<T> {
    map: HashMap<(usize, usize), T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new(height: usize, width: usize) -> Grid<T> {
        Grid {
            map: HashMap::new(),
            height,
            width,
        }
    }
}

fn build_input_map() -> Grid<char> {
    let mut grid = Grid::new(0, 0);
    for (y, line) in include_str!("../input/day03.txt").lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            grid.map.insert((y, x), val);
        }
        grid.height += 1;
        grid.width = line.chars().count();
    }
    grid
}

fn solve_slope(grid: &Grid<char>, dx: usize, dy: usize) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut tree_count = 0;

    while y < grid.height - 1 {
        y += dy;
        x = (x + dx) % grid.width;
        if *grid.map.get(&(y, x)).unwrap() == '#' {
            tree_count += 1;
        }
    }

    tree_count
}

pub fn solve_1() -> i64 {
    let grid = build_input_map();
    solve_slope(&grid, 3, 1)
}

pub fn solve_2() -> i64 {
    let grid = build_input_map();
    solve_slope(&grid, 1, 1)
        * solve_slope(&grid, 3, 1)
        * solve_slope(&grid, 5, 1)
        * solve_slope(&grid, 7, 1)
        * solve_slope(&grid, 1, 2)
}
