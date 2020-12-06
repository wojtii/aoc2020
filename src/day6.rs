use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;

pub fn run() {
    let input = read_to_string("src/day6-input.txt").unwrap();
    let groups: Vec<_> = input.split("\n\n").collect();

    let result = groups.iter().fold(0, |acc, x| {
        let unique: HashSet<_> =
            HashSet::from_iter(x.split_whitespace().collect::<String>().chars());
        acc + unique.len()
    });
    assert_eq!(6387, result);
    println!("{}", result);

    let result = groups.iter().fold(0, |acc, x| {
        let occ = x.split_whitespace().collect::<String>().chars().fold(
            HashMap::new(),
            |mut acc_inner, y| {
                *acc_inner.entry(y).or_insert(0) += 1;
                acc_inner
            },
        );
        let answers_count = x.lines().collect::<Vec<_>>().len();
        acc + occ.iter().fold(0, |acc_inner, (_, v)| {
            acc_inner + if v == &answers_count { 1 } else { 0 }
        })
    });
    assert_eq!(3039, result);
    println!("{}", result);
}
