use std::collections::HashSet;
use std::fs::read_to_string;

const WANT: i32 = 2020;

pub fn run() {
    let input = read_to_string("src/day1-input.txt").unwrap();
    let numbers: HashSet<_> = input
        .lines()
        .map(|x| x.parse())
        .filter_map(Result::ok)
        .map(|x| x)
        .collect();

    let result = numbers.iter().fold(0, |acc, x| {
        let complement = WANT - x;
        if numbers.contains(&complement) {
            x * complement
        } else {
            acc
        }
    });
    assert_eq!(866436, result);
    println!("{}", result);

    let result = numbers.iter().fold(0, |acc, n1| {
        let result_inner = numbers.iter().fold(0, |acc_inner, n2| {
            let complement = WANT - n1 - n2;
            if numbers.contains(&complement) {
                n1 * n2 * complement
            } else {
                acc_inner
            }
        });
        if result_inner > 0 {
            result_inner
        } else {
            acc
        }
    });
    println!("{}", result);
}
