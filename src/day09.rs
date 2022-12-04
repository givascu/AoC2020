fn build_input_vector() -> Vec<i64> {
    include_str!("../input/day09.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

pub fn solve_1() -> i64 {
    let v = build_input_vector();

    let width = 25;
    for k in width..v.len() {
        let mut found = false;
        for i in (k - width)..k {
            if found {
                break;
            }
            for j in (i + 1)..k {
                if v[i] + v[j] == v[k] {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return v[k];
        }
    }

    -1
}

fn try_solve_2(v: &Vec<i64>, target: i64, width: usize) -> i64 {
    for i in 0..=(v.len() - width) {
        let slice = &v[i..i + width];
        if slice.iter().sum::<i64>() == target {
            return slice.iter().min().unwrap() + slice.iter().max().unwrap();
        }
    }
    -1
}

pub fn solve_2() -> i64 {
    let target = solve_1();
    assert!(target != -1);

    let v = build_input_vector();
    for width in 2..v.len() {
        let res = try_solve_2(&v, target, width);
        if res != -1 {
            return res;
        }
    }

    -1
}
