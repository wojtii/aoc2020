use std::collections::HashSet;
use std::fs::read_to_string;

const WANT: i32 = 2020;

pub fn run() {
    let input = read_to_string("src/day1-input.txt").unwrap();
    let numbers = input.split("\n").map(|x| x.parse()).filter_map(Result::ok);

    let mut visited = HashSet::new();
    for n in numbers {
        let complement = WANT - n;
        if visited.contains(&complement) {
            let result = n * complement;
            assert_eq!(866436, result);
            println!("first part: {}", result);
        }
        visited.insert(n);
    }

    for n1 in &visited {
        for n2 in &visited {
            let complement = WANT - n1 - n2;
            if visited.contains(&complement) {
                let result = n1 * n2 * complement;
                assert_eq!(276650720, result);
                println!("second part: {}", result);
                return;
            }
        }
    }
}
