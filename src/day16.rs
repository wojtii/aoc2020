use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: (Range<u32>, Range<u32>),
}

impl Rule {
    fn in_range(&self, number: u32) -> bool {
        self.ranges.0.contains(&number) || self.ranges.1.contains(&number)
    }
}

pub fn run() {
    let input = read_to_string("src/day16-input.txt").unwrap();
    let parts: Vec<_> = input.split("\n\n").collect();

    let rules: Vec<_> = parts[0]
        .lines()
        .map(|x| {
            let name_ranges: Vec<_> = x.split(":").collect();
            let range: Vec<_> = name_ranges[1]
                .split("or")
                .map(|y| {
                    let lower_upper: Vec<u32> =
                        y.trim().split("-").map(|z| z.parse().unwrap()).collect();
                    lower_upper[0]..lower_upper[1] + 1
                })
                .collect();

            Rule {
                name: name_ranges[0].to_string(),
                ranges: (range[0].clone(), range[1].clone()),
            }
        })
        .collect();

    let nearby: Vec<Vec<_>> = parts[2]
        .lines()
        .skip(1) // remove "nearby tickets:"
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut invalid_nearby = vec![];
    let valid_nearby: Vec<_> = nearby
        .iter()
        .filter(|&x| {
            let invalid: Vec<_> = x
                .iter()
                .filter(|&y| rules.iter().fold(true, |acc, z| acc && !z.in_range(*y)))
                .collect();

            let is_invalid = invalid.len() == 0;
            invalid_nearby.extend(invalid);
            is_invalid
        })
        .collect();
    let result: u32 = invalid_nearby.iter().sum();
    assert_eq!(22977, result);
    println!("{}", result);

    let my_ticket: Vec<u32> = parts[1]
        .lines()
        .skip(1) // remove "your ticket:"
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .next()
        .unwrap();

    let n = rules.len();
    let mut all: Vec<HashSet<usize>> = vec![(0..n).collect(); n];

    valid_nearby.iter().for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(i, value)| {
            rules.iter().enumerate().for_each(|(j, rule)| {
                if !rule.in_range(*value) {
                    all[i].remove(&j);
                }
            });
        });
    });

    let mut occ = vec![0; n];
    let mut cnt = 0;
    while n > cnt {
        all.clone()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.len() == 1)
            .for_each(|(i, x)| {
                occ[i] = *x.iter().next().unwrap();
                cnt += 1;

                all.iter_mut().for_each(|y| {
                    y.remove(&occ[i]);
                });
            });
    }

    let departure_count = rules
        .iter()
        .filter(|x| x.name.starts_with("departure"))
        .count();

    let result: u64 = my_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| occ[*i] < departure_count)
        .map(|(_, &x)| x as u64)
        .product();

    assert_eq!(998358379943, result);
    println!("{}", result);
}
