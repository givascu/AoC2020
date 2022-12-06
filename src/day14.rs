use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Num36 {
    bits: [u8; 36],
}

impl Num36 {
    fn new(from: i32) -> Num36 {
        let mut from = from;
        let mut steps = 1_usize;
        let mut num = Num36 { bits: [0; 36] };

        while from > 0 {
            num.bits[36 - steps] = u8::try_from(from % 2).unwrap();
            from /= 2;
            steps += 1;
        }

        num
    }

    fn apply_mask(&mut self, mask: &str) {
        for (i, c) in mask.chars().enumerate() {
            if c == '1' {
                self.bits[i] = 1;
            } else if c == '0' {
                self.bits[i] = 0;
            }
        }
    }

    fn apply_floating_mask(&self, mask: &str) -> Vec<Num36> {
        let mut masked_self = *self;
        for (i, c) in mask.chars().enumerate() {
            if c == '1' {
                masked_self.bits[i] = 1;
            } else if c == 'X' {
                masked_self.bits[i] = 2;
            }
        }

        let mut retval = vec![];
        let mut stack = vec![masked_self];
        while !stack.is_empty() {
            let number = stack.pop().unwrap();
            for (i, &x) in number.bits.iter().enumerate() {
                if x == 2 {
                    let (mut n1, mut n2) = (number, number);
                    n1.bits[i] = 0;
                    stack.push(n1);
                    n2.bits[i] = 1;
                    stack.push(n2);
                    break;
                }
            }
            if number.bits.iter().all(|&x| x != 2) {
                retval.push(number);
            }
        }

        retval
    }

    fn value(&self) -> i64 {
        let mut val = 0;
        for (i, &x) in self.bits.iter().rev().enumerate() {
            val += 2_i64.pow(u32::try_from(i).unwrap()) * i64::from(x);
        }
        val
    }
}

pub fn solve_1() -> i64 {
    let mut memory = HashMap::new();
    let mut mask = "";

    for line in include_str!("../input/day14.txt").lines() {
        if line.starts_with("mask") {
            mask = line.split_once(" = ").unwrap().1;
        } else if line.starts_with("mem") {
            let parts = line.split_once(" = ").unwrap();
            let mut number = Num36::new(parts.1.parse::<i32>().unwrap());
            let address = parts
                .0
                .split_once('[')
                .unwrap()
                .1
                .trim_end_matches(']')
                .parse::<i32>()
                .unwrap();

            number.apply_mask(mask);
            memory.insert(address, number.value());
        }
    }

    memory.values().sum()
}

pub fn solve_2() -> i64 {
    let mut memory = HashMap::new();
    let mut mask = "";

    for line in include_str!("../input/day14.txt").lines() {
        if line.starts_with("mask") {
            mask = line.split_once(" = ").unwrap().1;
        } else if line.starts_with("mem") {
            let parts = line.split_once(" = ").unwrap();
            let number = parts.1.parse::<i64>().unwrap();
            let address = Num36::new(
                parts
                    .0
                    .split_once('[')
                    .unwrap()
                    .1
                    .trim_end_matches(']')
                    .parse::<i32>()
                    .unwrap(),
            );

            for addr in address.apply_floating_mask(mask) {
                memory.insert(addr.value(), number);
            }
        }
    }

    memory.values().sum()
}
