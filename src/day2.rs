use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day2-input.txt").unwrap();

    let result = input.lines().fold(0, |acc, x| {
        let (low, high, letter, password) = split(x);
        let count = password.matches(letter).count();
        if count >= low && count <= high {
            acc + 1
        } else {
            acc
        }
    });
    assert_eq!(528, result);
    println!("{}", result);

    let result = input.lines().fold(0, |acc, x| {
        let (low, high, letter, password) = split(x);
        let is_low_valid = password.chars().nth(low - 1) == Some(letter);
        let is_high_valid = password.chars().nth(high - 1) == Some(letter);
        if is_low_valid ^ is_high_valid {
            acc + 1
        } else {
            acc
        }
    });
    assert_eq!(497, result);
    println!("{}", result);
}

fn split(row: &str) -> (usize, usize, char, &str) {
    let parts: Vec<_> = row.split(" ").collect();
    let low_high: Vec<usize> = parts[0].split("-").filter_map(|x| x.parse().ok()).collect();
    let letter = parts[1].chars().next().unwrap();
    let password = parts[2];

    (low_high[0], low_high[1], letter, password)
}
