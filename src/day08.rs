use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    cmd: &'a str,
    arg: i64,
}

impl Instruction<'_> {
    fn new(cmd: &str, arg: i64) -> Instruction {
        Instruction { cmd, arg }
    }
}

fn execute(instructions: &HashMap<i64, Instruction>) -> (bool, i64) {
    let mut visited = HashSet::new();
    let mut acc = 0;

    let mut idx = 0;
    while idx < instructions.len() as i64 {
        let instruction = instructions.get(&idx).unwrap();
        if visited.contains(&idx) {
            return (false, acc);
        }
        visited.insert(idx);

        match instruction.cmd {
            "acc" => {
                acc += instruction.arg;
                idx += 1;
            }
            "jmp" => {
                idx += instruction.arg;
            }
            _ => idx += 1,
        }
    }

    (true, acc)
}

fn build_instruction_map() -> HashMap<i64, Instruction<'static>> {
    let mut instructions = HashMap::new();
    for (idx, line) in include_str!("../input/day08.txt").lines().enumerate() {
        let (cmd, arg) = line.split_once(' ').unwrap();
        instructions.insert(
            idx as i64,
            Instruction::new(cmd, arg.parse::<i64>().unwrap()),
        );
    }
    instructions
}

fn opposite(cmd: &str) -> &str {
    match cmd {
        "jmp" => "nop",
        "nop" => "jmp",
        _ => unreachable!(),
    }
}

pub fn solve_1() -> i64 {
    execute(&build_instruction_map()).1
}

pub fn solve_2() -> i64 {
    let mut instructions = build_instruction_map();

    for idx in 0..instructions.len() {
        let idx = idx as i64;
        let instruction = instructions.get(&idx).unwrap();
        let (cmd, arg) = (instruction.cmd, instruction.arg);
        if ["jmp", "nop"].contains(&cmd) {
            instructions.insert(idx, Instruction::new(opposite(cmd), arg));
            let (success, result) = execute(&instructions);
            if success {
                return result;
            }
            instructions.insert(idx, Instruction::new(cmd, arg));
        }
    }

    -1
}
