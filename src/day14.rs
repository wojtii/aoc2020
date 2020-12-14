use std::collections::HashMap;
use std::fs::read_to_string;

const B0: char = '0';
const B1: char = '1';
const BX: char = 'X';

#[derive(Debug)]
struct Instruction {
    mask: Vec<char>,
    updates: Vec<Update>,
}

#[derive(Debug)]
struct Update {
    address: u64,
    value: u64,
}

pub fn run() {
    let input = read_to_string("src/day14-input.txt").unwrap();
    let instructions: Vec<Instruction> = input
        .split("mask = ")
        .skip(1)
        .map(|x| {
            let lines: Vec<_> = x.lines().collect();
            let updates = lines[1..]
                .iter()
                .map(|y| {
                    let s: Vec<_> = y.split(" = ").collect();
                    let address: u64 = s[0]
                        .chars()
                        .skip(4)
                        .take(s[0].len() - 5)
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    let value: u64 = s[1].parse().unwrap();
                    Update { address, value }
                })
                .collect();
            Instruction {
                mask: lines[0].chars().collect(),
                updates,
            }
        })
        .collect();

    let mut mem = HashMap::new();
    for instruction in &instructions {
        for update in &instruction.updates {
            let value_bin: String = format!("{:036b}", update.value)
                .chars()
                .enumerate()
                .map(|(i, x)| match instruction.mask[i] {
                    B0 => B0,
                    B1 => B1,
                    BX => x,
                    y => unreachable!(format!("unexpected '{:?}'", y)),
                })
                .collect();

            mem.insert(
                update.address,
                u64::from_str_radix(&value_bin.to_owned(), 2).unwrap(),
            );
        }
    }
    let result: u64 = mem.values().sum();
    assert_eq!(18630548206046, result);
    println!("{}", result);

    let mut mem = HashMap::new();
    for instruction in &instructions {
        for update in &instruction.updates {
            let address_bin: Vec<_> = format!("{:036b}", update.address)
                .chars()
                .enumerate()
                .map(|(i, x)| match instruction.mask[i] {
                    B0 => x,
                    B1 => B1,
                    BX => BX,
                    y => unreachable!(format!("unexpected '{:?}'", y)),
                })
                .collect();

            let mut addresses = vec![address_bin.clone()];
            address_bin
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == BX)
                .for_each(|(i, _)| {
                    let mut next_addresses = Vec::with_capacity(addresses.len() * 2);
                    for a in &addresses {
                        for bit in [B0, B1].iter() {
                            let mut next_address = a.clone();
                            next_address[i] = *bit;
                            next_addresses.push(next_address);
                        }
                    }
                    addresses = next_addresses;
                });

            for a in addresses {
                let address_bin = a.iter().collect::<String>();
                let address = u64::from_str_radix(&address_bin.to_owned(), 2).unwrap();
                mem.insert(address, update.value);
            }
        }
    }
    let result: u64 = mem.values().sum();
    assert_eq!(4254673508445, result);
    println!("{}", result);
}
