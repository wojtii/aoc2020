use std::collections::HashSet;
use std::fs::read_to_string;

const NOP: &str = "nop";
const ACC: &str = "acc";
const JMP: &str = "jmp";

pub fn run() {
    let input = read_to_string("src/day8-input.txt").unwrap();
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .map(|x| {
            let mut s = x.split(" ");
            let instruction = s.next().unwrap();
            let value = s.next().unwrap();
            let value: i32 = value.parse().unwrap();
            (instruction, value)
        })
        .collect();

    let result = get_acc(&instructions);
    assert_eq!(1801, result);
    println!("{}", result);

    let result = fix(&instructions);
    assert_eq!(2060, result);
    println!("{}", result);
}

fn get_acc(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut result = 0;
    let mut idx: i32 = 0;
    let mut visited = HashSet::new();
    while (idx as usize) < instructions.len() && !visited.contains(&idx) {
        visited.insert(idx);
        let (ins, val) = instructions[idx as usize];
        match ins {
            NOP => idx += 1,
            ACC => {
                result += val;
                idx += 1;
            }
            JMP => idx += val,
            x => unreachable!(format!("unexpected '{}'", x)),
        }
    }

    result
}

fn fix(instructions: &Vec<(&str, i32)>) -> i32 {
    for (idx, (ins, val)) in instructions.iter().enumerate() {
        if *ins == ACC {
            continue;
        }

        let new_ins = match *ins {
            NOP => JMP,
            JMP => NOP,
            x => unreachable!(format!("unexpected '{}'", x)),
        };

        let mut copy = instructions.clone();
        copy[idx] = (new_ins, *val);
        if terminates(&copy) {
            return get_acc(&copy);
        }
    }

    unreachable!();
}

fn terminates(instructions: &Vec<(&str, i32)>) -> bool {
    let mut idx: i32 = 0;
    let mut visited = HashSet::new();
    while (idx as usize) < instructions.len() {
        if visited.contains(&idx) {
            return false;
        }

        visited.insert(idx);
        let (ins, val) = instructions[idx as usize];
        match ins {
            NOP | ACC => idx += 1,
            JMP => idx += val,
            x => unreachable!(format!("unexpected '{}'", x)),
        }
    }

    true
}
