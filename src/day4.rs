use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;

type Validator = fn(&str) -> bool;

pub fn run() {
    let input = read_to_string("src/day4-input.txt").unwrap();
    let validators: HashMap<&str, Validator> = vec![
        (
            "byr",
            (|v| validate_boundaries(v.to_owned(), 1920, 2002)) as Validator,
        ),
        ("iyr", |v| validate_boundaries(v.to_owned(), 2010, 2020)),
        ("eyr", |v| validate_boundaries(v.to_owned(), 2020, 2030)),
        ("hgt", |v| {
            let s: String = v.chars().skip(v.len() - 2).take(2).collect();
            let n: String = v.chars().take(v.len() - 2).collect();
            match s.to_owned().as_ref() {
                "cm" => validate_boundaries(n, 150, 193),
                "in" => validate_boundaries(n, 59, 76),
                _ => false,
            }
        }),
        ("hcl", |v| {
            Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(v)
        }),
        ("ecl", |v| {
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .into_iter()
                .collect::<HashSet<_>>()
                .contains(v)
        }),
        ("pid", |v| match v.parse::<i32>() {
            Ok(_) => v.len() == 9,
            Err(_) => false,
        }),
    ]
    .into_iter()
    .collect();
    let required_fields: HashSet<&str> = validators.keys().cloned().collect();

    let (result_first, result_second) = input.split("\n\n").fold((0, 0), |(first, second), x| {
        let person_fields: HashSet<_> =
            HashSet::from_iter(x.split_whitespace().map(|x| x.split(":").next().unwrap()));
        let difference = required_fields.difference(&person_fields);
        if difference.count() == 0 {
            let is_valid = x.split_whitespace().fold(true, |acc_inner, y| {
                let split: Vec<_> = y.split(":").collect();
                let (name, value) = (split[0], split[1]);
                match validators.get(name) {
                    Some(f) => acc_inner && f(value),
                    None => acc_inner,
                }
            });
            if is_valid {
                (first + 1, second + 1)
            } else {
                (first + 1, second)
            }
        } else {
            (first, second)
        }
    });
    assert_eq!(256, result_first);
    println!("{}", result_first);
    assert_eq!(198, result_second);
    println!("{}", result_second);
}

fn validate_boundaries(number: String, lower: i32, upper: i32) -> bool {
    match number.parse::<i32>() {
        Ok(v) => return v >= lower && v <= upper,
        Err(_) => return false,
    }
}
