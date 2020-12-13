use std::collections::HashMap;
use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day10-input.txt").unwrap();
    let mut nums: Vec<i32> = input.lines().filter_map(|x| x.parse().ok()).collect();
    nums.sort();
    nums.insert(0, 0); // wall adapter

    let (one_jolt_diffs, three_jolts_diffs) = nums[1..].iter().enumerate().fold(
        (0, 1), // count device joltage
        |(one_jolt_diffs, three_jolts_diffs), (i, num)| match num - nums[i] {
            1 => (one_jolt_diffs + 1, three_jolts_diffs),
            2 => (one_jolt_diffs, three_jolts_diffs),
            3 => (one_jolt_diffs, three_jolts_diffs + 1),
            x => unreachable!(format!("unexpected diff '{}'", x)),
        },
    );
    let result = one_jolt_diffs * three_jolts_diffs;
    assert_eq!(2470, result);
    println!("{}", result);

    let mut prevs: HashMap<i32, i64> = HashMap::new();
    prevs.insert(0, 1); // wall adapter
    for num in nums[1..].iter() {
        let count = (1..=3).fold(0, |acc, x| acc + prevs.get(&(num - x)).unwrap_or(&0));
        prevs.insert(*num, count);
    }
    let result = prevs.get(nums.last().unwrap()).unwrap();
    assert_eq!(1973822685184, *result);
    println!("{}", result);
}
